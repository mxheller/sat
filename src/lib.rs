#![feature(cmp_min_max_by, is_sorted, vec_remove_item)]

mod assignments;
mod conflict;
mod counters;
mod history;
mod literal;
mod luby;
mod sign;
mod solver;
pub mod trimmed_formula;
mod watched;

use assignments::{Assignment, Assignments};
use conflict::Conflict;
use counters::Counters;
use history::History;
use literal::Literal;
use luby::Luby;
pub use sign::Sign;
pub use solver::{Solution, Solver};
use watched::Watched;

pub type Variable = usize;
type ClauseIdx = usize;
type DecisionLevel = Variable;

trait Evaluate {
    fn evaluate(&self, assignments: &Assignments) -> Option<bool>;
}
