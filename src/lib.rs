mod assignments;
mod formula;
mod history;
mod literal;
mod sign;
mod solver;
mod trimmed_formula;
mod watched;

pub use assignments::{Assignment, Assignments};
pub use formula::Formula;
pub use history::History;
pub use literal::Literal;
pub use sign::Sign;
pub use solver::{Solution, Solver};
pub use watched::Watched;

pub type Variable = usize;
pub type ClauseIdx = usize;
pub type DecisionLevel = Variable;

pub trait Evaluate {
    fn evaluate(&self, assignments: &Assignments) -> Option<bool>;
}
