pub mod assignments;
pub mod evaluate;
pub mod formula;
pub mod sign;
pub mod solver;

pub type Variable = usize;
pub enum Status {
    Ok,
    Conflict,
}
pub enum Solution {
    Sat,
    Unsat,
}
pub type DecisionLevel = usize;
