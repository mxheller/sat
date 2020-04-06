use crate::{
    assignments::Assignments,
    formula::{Clause, Formula, Literal},
};

pub trait Evaluate {
    fn evaluate(&self, assignments: &Assignments) -> Option<bool>;
}

impl Evaluate for Literal {
    fn evaluate(&self, assignments: &Assignments) -> Option<bool> {
        unimplemented!();
    }
}

impl Evaluate for Clause {
    fn evaluate(&self, assignments: &Assignments) -> Option<bool> {
        unimplemented!();
    }
}

impl Evaluate for Formula {
    fn evaluate(&self, assignments: &Assignments) -> Option<bool> {
        unimplemented!();
    }
}
