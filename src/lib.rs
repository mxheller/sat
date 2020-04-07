mod assignments;
pub mod formula;
mod history;
mod literal;
mod sign;
mod solver;
mod trimmed_formula;
mod watched;

pub use assignments::{Assignment, Assignments};
pub use history::History;
pub use literal::Literal;
pub use sign::Sign;
pub use solver::Solver;
pub use watched::Watched;

pub type Variable = usize;
pub type ClauseIdx = usize;
pub type DecisionLevel = Variable;

pub trait Evaluate {
    fn evaluate(&self, assignments: &Assignments) -> Option<bool>;
}

pub enum Solution {
    Sat,
    Unsat,
}
