pub mod assignments;
pub mod formula;
pub mod history;
pub mod sign;
pub mod solver;
pub mod watched;

pub type Variable = usize;
pub type ClauseIdx = usize;
pub type DecisionLevel = Variable;

pub trait Evaluate {
    fn evaluate(&self, assignments: &assignments::Assignments) -> Option<bool>;
}

pub enum Status {
    Ok,
    Conflict(formula::Literals),
}
pub enum Solution {
    Sat,
    Unsat,
}
