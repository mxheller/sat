use crate::{
    formula::{clause, Formula},
    Assignment, Assignments, ClauseIdx, Conflict, Counters, DecisionLevel, History, Literal, Luby,
    Sign, Variable, Watched,
};
use rand::{
    distributions::{Bernoulli, Distribution},
    rngs::ThreadRng,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

const UNIT_RUN: usize = 100;
const RANDOM_VAR_FREQ: f64 = 0.02;

pub struct Solver {
    decision_level: usize,
    num_variables: usize,
    formula: Formula,
    counters: Counters<Variable>,
    assignments: Assignments,
    history: History,
    watched: Watched,
    conflict: Conflict,
    pending_update: Vec<ClauseIdx>,
    luby: Luby,
    num_conflicts: usize,
    next_restart: usize,
    rng: ThreadRng,
    random_branch: Bernoulli,
}

pub enum Solution<T: IntoIterator<Item = (Variable, Sign)>> {
    Sat(T),
    Unsat,
}

#[must_use]
#[derive(Debug)]
pub enum Status {
    Ok,
    Conflict(ConflictType),
    Unsat,
}

#[must_use]
#[derive(Debug)]
pub enum ConflictType {
    Clause(ClauseIdx),
    Literal(Literal),
}

impl Solver {
    pub fn parse_and_solve_file(
        path: impl AsRef<Path>,
    ) -> Result<Solution<impl IntoIterator<Item = (Variable, Sign)>>, String> {
        let lines = File::open(path)
            .map(|f| BufReader::new(f).lines().filter_map(Result::ok))
            .map_err(|e| format!("{}", e))?;

        Self::parse_and_solve(lines)
    }

    pub fn parse_and_solve(
        lines: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<Solution<impl IntoIterator<Item = (Variable, Sign)>>, String> {
        let mut lines = lines
            .into_iter()
            .skip_while(|l| l.as_ref().starts_with('c'));
        let problem_line = lines.next().ok_or_else(|| "No problem line".to_owned())?;
        let problem = problem_line.as_ref().split_whitespace().collect::<Vec<_>>();

        let clauses = lines.map(|l| {
            l.as_ref()
                .split_whitespace()
                .filter(|x| *x != "0")
                .map(|x| x.parse::<isize>().map(Literal::from))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("Invalid clause line: {}", e))
        });

        let (num_variables, num_clauses) = match problem.as_slice() {
            ["p", "cnf", vars, clauses] => (
                vars.parse().map_err(|e| format!("{}", e))?,
                clauses.parse().map_err(|e| format!("{}", e))?,
            ),
            _ => Err("Invalid problem line".to_owned())?,
        };

        let mut parsed_clauses = Vec::with_capacity(num_clauses);
        for clause in clauses {
            parsed_clauses.push(clause?);
        }

        Self::solve_clauses(parsed_clauses, num_variables).map(|solution| match solution {
            Solution::Unsat => Solution::Unsat,
            Solution::Sat(assignments) => {
                Solution::Sat(assignments.into_iter().map(|(var, sign)| (var + 1, sign)))
            }
        })
    }

    fn solve_clauses(
        clauses: Vec<Vec<Literal>>,
        num_variables: usize,
    ) -> Result<Solution<impl IntoIterator<Item = (Variable, Sign)>>, String> {
        let mut luby = Luby::new();

        let mut solver = Self {
            formula: Formula::new(clauses.len()),
            counters: Counters::new(num_variables),
            assignments: Assignments::new(num_variables),
            history: History::new(num_variables),
            watched: Watched::new(num_variables),
            conflict: Conflict::new(num_variables),
            pending_update: Vec::with_capacity(clauses.len()),
            decision_level: 0,
            num_variables,
            next_restart: luby.next() * UNIT_RUN,
            num_conflicts: 0,
            luby,
            rng: rand::thread_rng(),
            random_branch: Bernoulli::new(RANDOM_VAR_FREQ).unwrap(),
        };

        // Add clauses to formula
        for clause in clauses {
            match solver.learn_clause(clause.into_iter())? {
                Status::Ok => (),
                _ => return Ok(Solution::Unsat),
            }
        }

        solver.solve()
    }

    fn solve(mut self) -> Result<Solution<impl IntoIterator<Item = (Variable, Sign)>>, String> {
        loop {
            match self.propogate_all() {
                Status::Ok => {
                    if self.all_variables_assigned() {
                        return Ok(Solution::Sat(self.assignments.assignments()));
                    }
                    self.branch()?;
                }
                Status::Unsat => return Ok(Solution::Unsat),
                Status::Conflict(conflict) => {
                    self.counters.decay_activity();
                    self.num_conflicts += 1;
                    let restart = self.num_conflicts == self.next_restart;
                    if restart {
                        self.num_conflicts = 0;
                        self.next_restart = self.luby.next() * UNIT_RUN;
                    }

                    let status = match conflict {
                        ConflictType::Literal(literal) => {
                            Conflict::var_backtrack_level(literal.var(), &self.assignments).map(
                                |level| {
                                    if restart {
                                        self.backtrack(0);
                                    } else {
                                        self.backtrack(level);
                                    }
                                    self.learn_clause(std::iter::once(literal))
                                },
                            )
                        }
                        ConflictType::Clause(clause) => {
                            self.analyze_conflict(clause)?.map(|(learned, level)| {
                                if restart {
                                    self.backtrack(0);
                                } else {
                                    self.backtrack(level);
                                }
                                self.learn_clause(learned.into_iter())
                            })
                        }
                    };
                    match status {
                        None => return Ok(Solution::Unsat),
                        Some(status) => assert!(matches!(status?, Status::Ok)),
                    }
                }
            }
        }
    }

    fn new_decision_level(&mut self) {
        self.decision_level += 1;
        self.history.new_decision_level();
    }

    fn assign_decided(&mut self, literal: Literal) -> Status {
        self.assignments.set(
            literal.var(),
            Assignment::decided(literal.sign(), self.decision_level),
            &mut self.history,
        )
    }

    fn assign_implied(&mut self, literal: Literal, antecedent: ClauseIdx) -> Status {
        self.assignments.set(
            literal.var(),
            Assignment::implied(literal.sign(), antecedent, self.decision_level),
            &mut self.history,
        )
    }

    fn assign_invariant(&mut self, literal: Literal) -> Status {
        self.assignments
            .set_invariant(literal.var(), literal.sign(), &mut self.history)
    }

    fn propogate_all(&mut self) -> Status {
        while let Some(literal) = self.history.next_to_propogate() {
            match self.propogate(literal) {
                Status::Ok => (),
                status => return status,
            }
        }
        Status::Ok
    }

    fn propogate(&mut self, literal: Literal) -> Status {
        // Find clauses in which negated literal (now unsatisfied) is watched
        self.pending_update.extend(self.watched[!literal].iter());

        while let Some(clause) = self.pending_update.pop() {
            match self.formula[clause].update(&mut self.watched, &self.assignments, clause) {
                clause::Status::Ok => (),
                clause::Status::Conflict => return Status::Conflict(ConflictType::Clause(clause)),
                clause::Status::Implied(literal) => match self.assign_implied(literal, clause) {
                    Status::Ok => (),
                    status => return status,
                },
            }
        }

        Status::Ok
    }

    fn learn_clause(
        &mut self,
        mut clause: impl Iterator<Item = Literal> + ExactSizeIterator,
    ) -> Result<Status, String> {
        match clause.len() {
            0 => Ok(Status::Unsat),
            1 => {
                let unit = clause.next().unwrap();
                self.counters.bump(unit.var());
                Ok(self.assign_invariant(unit))
            }
            _ => {
                let (clause, status) = self.formula.add_clause(
                    clause,
                    &mut self.watched,
                    &mut self.counters,
                    &self.assignments,
                )?;

                Ok(match status {
                    clause::Status::Ok => Status::Ok,
                    clause::Status::Conflict => Status::Conflict(ConflictType::Clause(clause)),
                    clause::Status::Implied(literal) => self.assign_implied(literal, clause),
                })
            }
        }
    }

    fn all_variables_assigned(&self) -> bool {
        self.history.num_assigned() == self.num_variables
    }

    fn branch(&mut self) -> Result<(), String> {
        self.new_decision_level();
        let err = || "No variable to branch on".to_owned();

        // Get an initial choice, possibly randomly
        let mut var = if self.random_branch.sample(&mut self.rng) {
            self.counters.random_var(&mut self.rng)
        } else {
            self.counters.next_var()
        }
        .ok_or_else(err)?;

        // Get new choices until one is not already assigned
        while self.assignments[var].is_some() {
            var = self.counters.next_var().ok_or_else(err)?;
        }

        match self.assign_decided(Literal::new(var, self.assignments.last_sign(var))) {
            Status::Ok => Ok(()),
            _ => Err("Branched on already decided variable".to_owned()),
        }
    }

    fn analyze_conflict(
        &mut self,
        conflict_clause: ClauseIdx,
    ) -> Result<Option<(Vec<Literal>, DecisionLevel)>, String> {
        let (level, assignments) = (self.decision_level, &self.assignments);
        if level == 0 {
            return Ok(None);
        }

        let conflict = &mut self.conflict;
        conflict.initialize(
            self.decision_level,
            &self.formula[conflict_clause],
            assignments,
        );

        for literal in self.history.most_recently_implied_at_current_level() {
            if conflict.assigned_at_level() <= 1 {
                break;
            }
            if conflict.contains(!literal) {
                let antecedent = assignments
                    .get(literal.var())
                    .and_then(Assignment::antecedent)
                    .ok_or_else(|| {
                        eprintln!(
                            "Assignment at level {}: {:?}",
                            level,
                            assignments.get(literal.var())
                        );
                        format!(
                            "Supposedly implied literal {} was unassigned or had no antecedent",
                            literal
                        )
                    })?;
                conflict.resolve(!literal, &self.formula[antecedent], assignments)?;
            }
        }

        Ok(conflict
            .backtrack_level(level, assignments)
            .map(|level| (conflict.literals().collect(), level)))
    }

    fn backtrack(&mut self, level: usize) {
        self.decision_level = level;
        self.history
            .revert_to(level, &mut self.assignments, &mut self.counters);
    }
}

#[test]
fn all_variables_assigned_after_propogating() -> Result<(), String> {
    let clauses = vec![
        vec![Literal::new(0, true), Literal::new(1, true)],
        vec![Literal::new(0, false), Literal::new(1, true)],
    ];
    match Solver::solve_clauses(clauses, 2)? {
        Solution::Unsat => Err("Expected Sat, got Unsat".to_string()),
        Solution::Sat(assignment) => {
            let assignments = assignment.into_iter().collect::<Vec<_>>();
            assert!(assignments
                .iter()
                .any(|(var, sign)| *var == 1 && *sign == Sign::Positive));
            assert!(assignments.iter().any(|(var, _)| *var == 0));
            Ok(())
        }
    }
}

#[test]
fn minimal_unsat() {
    let clauses = vec![vec![Literal::new(0, true)], vec![Literal::new(0, false)]];
    assert!(matches!(
        Solver::solve_clauses(clauses, 1),
        Ok(Solution::Unsat)
    ));
}

#[test]
fn unsat() {
    let clauses = vec![
        vec![Literal::new(0, true), Literal::new(1, true)],
        vec![Literal::new(0, true), Literal::new(1, false)],
        vec![Literal::new(0, false), Literal::new(1, true)],
        vec![Literal::new(0, false), Literal::new(1, false)],
    ];
    assert!(matches!(
        Solver::solve_clauses(clauses, 2),
        Ok(Solution::Unsat)
    ));
}

#[test]
fn zebra() {
    let solution = Solver::parse_and_solve_file("inputs/zebra.cnf");
    assert!(matches!(solution, Ok(Solution::Sat(_))));
}

#[test]
fn dubois() {
    let solution = Solver::parse_and_solve_file("inputs/dubois.cnf");
    assert!(matches!(solution, Ok(Solution::Unsat)));
}

#[test]
fn aim100() {
    let solution = Solver::parse_and_solve_file("inputs/aim-100.cnf");
    assert!(matches!(solution, Ok(Solution::Unsat)));
}

#[test]
fn aim50() {
    let solution = Solver::parse_and_solve_file("inputs/aim-50.cnf");
    assert!(matches!(solution, Ok(Solution::Sat(_))));
}

#[test]
fn bf() {
    let solution = Solver::parse_and_solve_file("inputs/bf0432-007.cnf");
    assert!(matches!(solution, Ok(Solution::Unsat)));
}
