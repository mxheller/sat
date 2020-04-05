use crate::{assignments::Assignments, Status};

pub mod clause;
pub mod literal;

pub use clause::Clause;
pub use literal::Literal;

pub struct Formula(Vec<Clause>);

impl Formula {
    pub fn perform_unit_propogation(&mut self, assignments: &mut Assignments) -> Status {
        unimplemented!()
    }
}
