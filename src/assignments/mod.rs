use crate::{History, Sign, Variable};

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

    pub fn get(&self, var: Variable) -> Option<&Assignment> {
        self.assignments[var as usize].as_ref()
    }

    pub fn set(&mut self, var: Variable, assignment: Assignment, history: &mut History) {
        assert!(matches!(self.get(var), None));
        self.assignments[var] = Some(assignment);
        history.add(var);
    }

    pub(crate) fn set_invariant(&mut self, var: Variable, sign: Sign) {
        assert!(matches!(self.get(var), None));
        self.assignments[var] = Some(Assignment::decided(sign, 0));
    }

    pub fn remove(&mut self, var: Variable) {
        self.assignments[var] = None;
    }
}
