mod assignments;
mod formula;
mod history;
mod literal;
mod sign;
mod solver;
mod trimmed_formula;
mod watched;

use assignments::{Assignment, Assignments};
pub use formula::Formula;
use history::History;
use literal::Literal;
pub use sign::Sign;
pub use solver::{Solution, Solver};
use watched::Watched;

pub type Variable = usize;
type ClauseIdx = usize;
type DecisionLevel = Variable;

trait Evaluate {
    fn evaluate(&self, assignments: &Assignments) -> Option<bool>;
}
