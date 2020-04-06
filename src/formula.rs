use crate::{assignments::Assignments, Status, Variable};

pub mod clause;
pub mod literal;

pub use clause::Clause;
pub use literal::Literal;

pub struct Formula {
    pub clauses: Vec<Clause>,
}

impl Formula {
    pub fn perform_unit_propogation(&mut self, assignments: &mut Assignments) -> Status {
        unimplemented!()
    }

    pub fn num_variables(&self) -> Variable {
        unimplemented!()
    }
}
