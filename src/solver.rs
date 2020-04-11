use crate::{
    formula,
    trimmed_formula::{clause, TrimmedFormula},
    Assignment, Assignments, ClauseIdx, Conflict, Counters, DecisionLevel, History, Literal, Sign,
    Variable, Watched,
};
use std::collections::BTreeMap;

pub struct Solver {
    decision_level: usize,
    num_variables: usize,
    formula: TrimmedFormula,
    counters: Counters,
    assignments: Assignments,
    history: History,
    watched: Watched,
    conflict: Conflict,
}

pub enum Solution<T: IntoIterator<Item = (Variable, Sign)>> {
    Sat(T),
    Unsat,
}

#[must_use]
pub enum Status {
    Ok,
    ConflictClause(ClauseIdx),
    ConflictLiteral(Literal),
    Unsat,
}

impl Solver {
    pub fn solve_formula(
        formula: impl Into<formula::Formula>,
    ) -> Result<Solution<impl IntoIterator<Item = (Variable, Sign)>>, String> {
        let formula = formula.into();

        // Create mapping from [0, numVars) -> Variable
        let variables = formula.distinct_variables();

        // Create inverse mapping from Variable -> [0, numVars)
        let map = variables
            .iter()
            .enumerate()
            .map(|(idx, id)| (*id, idx as Variable))
            .collect::<BTreeMap<Variable, Variable>>();

        let num_variables = variables.len();

        let mut solver = Self {
            formula: TrimmedFormula::new(formula.clauses.len()),
            counters: Counters::new(num_variables),
            assignments: Assignments::new(num_variables),
            history: History::new(num_variables),
            watched: Watched::new(num_variables),
            conflict: Conflict::new(num_variables),
            decision_level: 0,
            num_variables,
        };

        for clause in formula.clauses.into_iter() {
            // Convert literals to new numbering
            let literals = clause
                .into_iter()
                .map(|literal| Literal::new(map[&literal.var()], literal.sign()));

            // Add clause to trimmed formula
            match solver.learn_clause(literals)? {
                Status::Ok => (),
                _ => return Ok(Solution::Unsat),
            }
        }

        Ok(match solver.solve()? {
            Solution::Unsat => Solution::Unsat,
            Solution::Sat(assignments) => Solution::Sat(
                assignments
                    .into_iter()
                    .map(move |(var, sign)| (variables[var as usize], sign)),
            ),
        })
    }

    fn solve(mut self) -> Result<Solution<impl IntoIterator<Item = (Variable, Sign)>>, String> {
        while !self.all_variables_assigned() {
            match self.propogate_all() {
                Status::Ok => {
                    if !self.all_variables_assigned() {
                        self.branch()?;
                    }
                }
                Status::Unsat => return Ok(Solution::Unsat),
                Status::ConflictLiteral(literal) => {
                    let level = self
                        .assignments
                        .get(literal.var())
                        .unwrap()
                        .decision_level();

                    if level == 0 {
                        return Ok(Solution::Unsat);
                    }

                    self.backtrack(level - 1);
                    assert!(matches!(
                        self.learn_clause(std::iter::once(literal)),
                        Ok(Status::Ok)
                    ));
                }
                Status::ConflictClause(clause) => match self.analyze_conflict(clause)? {
                    Some((learned, level)) => {
                        self.backtrack(level);
                        assert!(matches!(
                            self.learn_clause(learned.into_iter()),
                            Ok(Status::Ok)
                        ));
                    }
                    None => return Ok(Solution::Unsat),
                },
            }
        }

        Ok(Solution::Sat(self.assignments.assignments()))
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
        let affected_clauses = self.watched[!literal]
            .iter()
            .copied()
            .collect::<Vec<usize>>();

        for clause in affected_clauses {
            match self.formula[clause].update(&mut self.watched, &self.assignments, clause) {
                clause::Status::Ok => (),
                clause::Status::Conflict => return Status::ConflictClause(clause),
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
                self.counters.increment(unit);
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
                    clause::Status::Conflict => Status::ConflictClause(clause),
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
        let choice = self
            .counters
            .next_decision(&self.assignments)
            .ok_or_else(|| "No variable to branch on".to_owned())?;
        dbg!(choice);
        match self.assign_decided(choice) {
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
            println!("Checking {}", literal);
            dbg!(conflict.assigned_at_level());
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
        self.history.revert_to(level, &mut self.assignments);
    }
}

#[test]
fn all_variables_assigned_after_propogating() -> Result<(), String> {
    let formula: formula::Formula = vec![
        vec![
            Literal::new(0, Sign::Positive),
            Literal::new(1, Sign::Positive),
        ],
        vec![
            Literal::new(0, Sign::Negative),
            Literal::new(1, Sign::Positive),
        ],
    ]
    .into();
    match formula.solve()? {
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
fn learning() -> Result<(), String> {
    let formula: formula::Formula = vec![
        vec![4, 2, -5],
        vec![4, -6],
        vec![5, 6, 7],
        vec![-7, -8],
        vec![1, -7, -9],
        vec![8, 9],
    ]
    .into();
    formula.solve().map(|_| ())
}
