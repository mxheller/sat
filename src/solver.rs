use crate::{
    assignments::{Assignment, Assignments},
    formula::{Clause, Formula},
    sign::Sign,
    DecisionLevel, Solution, Status, Variable,
};

pub struct Solver<'a> {
    formula: Formula,
    assignments: Assignments<'a>,
    decision_level: usize,
}

impl Solver<'_> {
    pub fn new(formula: Formula) -> Self {
        Self {
            formula,
            assignments: Assignments::new(),
            decision_level: 0,
        }
    }

    pub fn solve(mut self) -> Solution {
        if let Status::Conflict(_) = self.perform_unit_propogation() {
            return Solution::Unsat;
        }

        while !self.all_variables_assigned() {
            let (var, sign) = self.pick_branching_variable();
            self.decision_level += 1;
            self.assignments[var] = Some(Assignment::decided(sign, self.decision_level));

            if let Status::Conflict(c) = self.perform_unit_propogation() {
                if let Some((learned, level)) = self.analyze_conflict(c) {
                    self.formula.add(learned);
                    self.backtrack(level);
                } else {
                    return Solution::Unsat;
                }
            }
        }

        return Solution::Sat;
    }

    fn perform_unit_propogation(&mut self) -> Status {
        unimplemented!()
    }

    fn all_variables_assigned(&self) -> bool {
        unimplemented!()
    }

    fn pick_branching_variable(&self) -> (Variable, Sign) {
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
            clause.resolve(&antecedent.unwrap());
        }

        // TODO: is this the correct level to backtrack to?
        let backtrack_level = clause.asserting_level(assignments);
        Some((clause, backtrack_level))
    }

    fn backtrack(&mut self, level: usize) {
        self.decision_level = level;
        unimplemented!()
    }
}
