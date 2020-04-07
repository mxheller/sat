use crate::{history::History, Variable};
use std::ops::Index;

pub mod assignment;
pub use assignment::Assignment;

#[derive(Clone, Debug)]
pub struct Assignments {
    assignments: Vec<Option<Assignment>>,
}

impl Assignments {
    pub fn new(num_vars: Variable) -> Self {
        Self {
            assignments: vec![None; num_vars as usize],
        }
    }

    pub fn implies(&self, a: Variable, b: Variable) -> bool {
        unimplemented!();
    }

    pub fn set(&mut self, var: Variable, assignment: Assignment, history: &mut History) {
        assert!(matches!(self[var], None));
        self.assignments[var] = Some(assignment);
        history.add(var);
    }

    pub fn remove(&mut self, var: Variable) {
        self.assignments[var] = None;
    }
}

impl Index<Variable> for Assignments {
    type Output = Option<Assignment>;

    #[inline]
    fn index(&self, var: Variable) -> &Self::Output {
        &self.assignments[var as usize]
    }
}
