use crate::{
    formula,
    trimmed_formula::{clause, TrimmedFormula},
    Assignment, Assignments, ClauseIdx, DecisionLevel, History, Literal, Sign, Variable, Watched,
};
use std::collections::BTreeMap;

pub struct Solver {
    decision_level: usize,
    num_variables: usize,
    formula: TrimmedFormula,
    assignments: Assignments,
    history: History,
    watched: Watched,
}

pub enum Solution<T: IntoIterator<Item = (Variable, Sign)>> {
    Sat(T),
    Unsat,
}

#[must_use]
pub enum Status {
    Ok,
    Conflict(formula::Clause),
}

impl Solver {
    pub fn solve_formula(
        formula: impl Into<formula::Formula>,
    ) -> Solution<impl IntoIterator<Item = (Variable, Sign)>> {
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
            assignments: Assignments::new(num_variables),
            history: History::new(num_variables),
            watched: Watched::new(num_variables),
            decision_level: 0,
            num_variables,
        };

        for clause in formula.clauses.into_iter() {
            // Convert literals to new numbering
            let literals = clause
                .into_iter()
                .map(|literal| Literal::new(map[&literal.var()], literal.sign()));

            // Add clause to trimmed formula
            if let Status::Conflict(_) = solver.learn_clause(literals) {
                return Solution::Unsat;
            }
        }

        match solver.solve() {
            Solution::Unsat => Solution::Unsat,
            Solution::Sat(assignments) => Solution::Sat(
                assignments
                    .into_iter()
                    .map(move |(var, sign)| (variables[var as usize], sign)),
            ),
        }
    }

    fn solve(mut self) -> Solution<impl IntoIterator<Item = (Variable, Sign)>> {
        while !self.all_variables_assigned() {
            match self.propogate_all() {
                Status::Ok => {
                    let choice = self.pick_branching_variable();
                    assert!(matches!(self.assign_decided(choice), Status::Ok));
                }
                Status::Conflict(c) => match self.analyze_conflict(c) {
                    Some((learned, level)) => {
                        println!("backtracking to {}, learning {:?}", level, learned);
                        self.backtrack(level);
                        assert!(!matches!(
                            self.learn_clause(learned.into_iter()),
                            Status::Conflict(_)
                        ));
                    }
                    None => return Solution::Unsat,
                },
            }
        }

        Solution::Sat(self.assignments.assignments())
    }

    fn new_decision_level(&mut self) {
        self.decision_level += 1;
        self.history.new_decision_level();
    }

    fn assign_decided(&mut self, literal: Literal) -> Status {
        self.assignments.set(
            literal.var(),
            Assignment::decided(literal.sign(), self.decision_level),
            &self.formula,
            &mut self.history,
        )
    }

    fn assign_implied(&mut self, literal: Literal, antecedent: ClauseIdx) -> Status {
        self.assignments.set(
            literal.var(),
            Assignment::implied(literal.sign(), antecedent, self.decision_level),
            &self.formula,
            &mut self.history,
        )
    }

    fn assign_invariant(&mut self, literal: Literal) -> Status {
        self.assignments.set_invariant(
            literal.var(),
            literal.sign(),
            &self.formula,
            &mut self.history,
        )
    }

    fn propogate_all(&mut self) -> Status {
        while let Some(literal) = self.history.next_to_propogate() {
            if let c @ Status::Conflict(_) = self.propogate(literal) {
                return c;
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
                clause::Status::Conflict(literals) => return Status::Conflict(literals),
                clause::Status::Implied(literal) => {
                    if let c @ Status::Conflict(_) = self.assign_implied(literal, clause) {
                        return c;
                    }
                }
            }
        }

        Status::Ok
    }

    fn learn_clause(
        &mut self,
        mut clause: impl Iterator<Item = Literal> + ExactSizeIterator,
    ) -> Status {
        match clause.len() {
            0 => panic!("Trying to learn an empty (unsat) clause"),
            1 => {
                let unit = clause.next().unwrap();
                self.assign_invariant(unit)
            }
            _ => {
                let (clause, status) =
                    self.formula
                        .add_clause(clause, &mut self.watched, &self.assignments);

                match status {
                    clause::Status::Ok => Status::Ok,
                    clause::Status::Conflict(c) => Status::Conflict(c),
                    clause::Status::Implied(literal) => self.assign_implied(literal, clause),
                }
            }
        }
    }

    fn all_variables_assigned(&self) -> bool {
        self.history.num_assigned() == self.num_variables
    }

    fn pick_branching_variable(&mut self) -> Literal {
        self.new_decision_level();
        // TODO: smart
        (0..self.num_variables)
            .find(|var| matches!(self.assignments.get(*var), None))
            .map(|var| Literal::new(var, true))
            .unwrap()
    }

    fn analyze_conflict(
        &self,
        mut clause: formula::Clause,
    ) -> Option<(formula::Clause, DecisionLevel)> {
        let (level, assignments) = (self.decision_level, &self.assignments);

        println!("\n\nconflict at level {}: {:?}", level, &clause);
        println!(
            "assignments: {:?}",
            clause.assignments(assignments).collect::<Vec<_>>()
        );

        if level == 0 {
            // Cannot backtrack any farther
            return None;
        }

        // Ensure there is at least one literal assigned at the conflict level
        debug_assert_ne!(
            clause.variables()
                .filter(|var| assignments.get(*var).unwrap().decision_level() == level)
                .count(),
            0,
            "There should be at least one literal assigned at the conflict level in the conflict clause"
        );

        loop {
            let mut at_level = clause.assignments_at(level, assignments);
            if let (Some(a), Some(b)) = (at_level.next(), at_level.next()) {
                let (literal, assignment) = match (a.1.antecedent(), b.1.antecedent()) {
                    (Some(_), _) => a,
                    (None, Some(_)) => b,

                    // Would mean we decided two vars at current level
                    (None, None) => unreachable!(),
                };
                let antecedent = assignment.antecedent().unwrap();
                drop(at_level);
                clause.resolve(literal, &self.formula[antecedent]);
            } else {
                // At most one literal assigned at conflict level
                break;
            }
        }

        clause
            .backtrack_level(level, assignments)
            .map(|level| (clause, level))
    }

    fn backtrack(&mut self, level: usize) {
        self.decision_level = level;
        self.history.revert_to(level, &mut self.assignments);
    }
}
