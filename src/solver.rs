use crate::{
    assignments::{Assignment, Assignments},
    formula::Formula,
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
        if let Status::Conflict = self.perform_unit_propogation() {
            return Solution::Unsat;
        }

        while !self.all_variables_assigned() {
            let (var, sign) = self.pick_branching_variable();
            self.decision_level += 1;
            self.assignments[var] = Some(Assignment::decided(sign, self.decision_level));

            if let Status::Conflict = self.perform_unit_propogation() {
                if let Some(level) = self.analyze_conflict() {
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

    fn analyze_conflict(&self) -> Option<DecisionLevel> {
        unimplemented!()
    }

    fn backtrack(&mut self, level: usize) {
        self.decision_level = level;
        unimplemented!()
    }
}
