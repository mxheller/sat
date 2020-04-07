pub mod assignments;
pub mod evaluate;
pub mod formula;
pub mod history;
pub mod sign;
pub mod solver;
pub mod watched;

pub type Variable = usize;
pub type ClauseIdx = usize;

pub enum Status {
    Ok,
    Conflict(formula::Clause),
}
pub enum Solution {
    Sat,
    Unsat,
}
pub type DecisionLevel = usize;
