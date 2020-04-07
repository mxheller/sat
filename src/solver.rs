use crate::{
    assignments::{Assignment, Assignments},
    formula::{
        clause::{Clause, ClauseUpdateResult},
        Formula, Literal,
    },
    history::History,
    watched::Watched,
    ClauseIdx, DecisionLevel, Solution, Status,
};

pub struct Solver {
    decision_level: usize,
    formula: Formula,
    assignments: Assignments,
    history: History,
    watched: Watched,
}

impl Solver {
    pub fn new(formula: impl Into<Formula>) -> Self {
        let formula = formula.into();
        let num_vars = formula.num_variables();
        Self {
            formula,
            decision_level: 0,
            assignments: Assignments::new(num_vars),
            history: History::new(num_vars),
            watched: Watched::new(num_vars),
        }
    }

    pub fn solve(mut self) -> Solution {
        if let Status::Conflict(_) = self.perform_unit_propogation() {
            return Solution::Unsat;
        }

        while !self.all_variables_assigned() {
            let literal = self.pick_branching_variable();
            self.decision_level += 1;
            self.assign_and_propogate_decided(literal);

            if let Status::Conflict(c) = self.perform_unit_propogation() {
                if let Some((learned, level)) = self.analyze_conflict(c) {
                    self.formula.clauses.push(learned);
                    self.backtrack(level);
                } else {
                    return Solution::Unsat;
                }
            }
        }

        Solution::Sat
    }

    fn assign_and_propogate_decided(&mut self, literal: Literal) -> Status {
        self.assignments.set(
            literal.var(),
            Assignment::decided(literal.sign(), self.decision_level),
            &mut self.history,
        );
        self.propogate(literal)
    }

    fn assign_implied(&mut self, literal: Literal, antecedent: ClauseIdx) {
        self.assignments.set(
            literal.var(),
            Assignment::implied(literal.sign(), antecedent, self.decision_level),
            &mut self.history,
        );
        self.history.add(literal.var());
    }

    fn propogate(&mut self, literal: Literal) -> Status {
        let mut implied = Vec::new();

        // Find clauses in which negated literal (now unsatisfied) is watched
        let affected_clauses = self.watched[!literal]
            .iter()
            .copied()
            .collect::<Vec<usize>>();

        for clause in affected_clauses {
            match self.formula.clauses[clause].update(&mut self.watched, &self.assignments, clause)
            {
                ClauseUpdateResult::Ok => (),
                ClauseUpdateResult::Conflict(clause) => return Status::Conflict(clause),
                ClauseUpdateResult::Implied(literal) => {
                    self.assign_implied(literal, clause);
                    implied.push(literal);
                }
            }
        }

        for literal in implied.into_iter() {
            if let c @ Status::Conflict(_) = self.propogate(literal) {
                return c;
            }
        }

        Status::Ok
    }

    fn perform_unit_propogation(&mut self) -> Status {
        unimplemented!()
    }

    fn all_variables_assigned(&self) -> bool {
        unimplemented!()
    }

    fn pick_branching_variable(&self) -> Literal {
        unimplemented!()
    }

    fn analyze_conflict(&self, mut clause: Clause) -> Option<(Clause, DecisionLevel)> {
        let (level, assignments) = (self.decision_level, &self.assignments);

        if level == 0 {
            return None;
        }

        while clause.literals_assigned_at(level, assignments).count() > 1 {
            // TODO: this ^ conditional can probably be combined with the below
            // section using equation 4.18

            let antecedent = clause
                .literals()
                .find_map(|literal| literal.implied_in_at_level(&clause, level, assignments));
            clause.resolve(&self.formula.clauses[antecedent.unwrap()]);
        }

        // TODO: is this the correct level to backtrack to?
        let backtrack_level = clause.asserting_level(assignments);
        Some((clause, backtrack_level))
    }

    fn backtrack(&mut self, level: usize) {
        self.decision_level = level;
        self.history.revert_to(level, &mut self.assignments);
    }
}
