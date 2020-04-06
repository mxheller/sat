use crate::{
    assignments::Assignments,
    formula::{Clause, Formula, Literal},
    DecisionLevel,
};

pub trait Evaluate {
    fn evaluate(&self, level: DecisionLevel, assignments: &Assignments) -> Option<bool>;
}

impl Evaluate for Literal {
    fn evaluate(&self, level: DecisionLevel, assignments: &Assignments) -> Option<bool> {
        assignments
            .get(self.var(), level)
            .map(|assignment| assignment.sign() == self.sign())
    }
}

impl Evaluate for Clause {
    fn evaluate(&self, level: DecisionLevel, assignments: &Assignments) -> Option<bool> {
        unimplemented!();
    }
}

impl Evaluate for Formula {
    fn evaluate(&self, level: DecisionLevel, assignments: &Assignments) -> Option<bool> {
        unimplemented!();
    }
}
