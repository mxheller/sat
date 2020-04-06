use crate::{DecisionLevel, Variable};
use std::marker::PhantomData;

pub mod assignment;
pub use assignment::Assignment;

pub struct Assignments {
    assignments: Vec<Option<Assignment>>,
    lifetime: PhantomData<Assignment>,
}

impl Assignments {
    pub fn new(num_vars: Variable) -> Self {
        Self {
            assignments: Vec::with_capacity(num_vars),
            lifetime: PhantomData,
        }
    }

    pub fn implies(&self, a: Variable, b: Variable) -> bool {
        unimplemented!();
    }

    /// Get the assignment of a variable as of a given decision level
    pub fn get(&self, var: Variable, current_level: DecisionLevel) -> Option<&Assignment> {
        self.assignments[var].as_ref().and_then(|assignment| {
            // Don't return assignment if it was made after current decision level
            if assignment.decision_level() > current_level {
                None
            } else {
                Some(assignment)
            }
        })
    }

    pub fn set(&mut self, var: Variable, assignment: Assignment) {
        debug_assert!(matches!(self.get(var, assignment.decision_level()), None));
        self.assignments[var] = Some(assignment);
    }
}
