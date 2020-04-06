use crate::{
    formula::{Clause, Formula, Literal},
    sign::Sign,
    Variable,
};
use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

pub mod assignment;
pub use assignment::Assignment;

pub struct Assignments<'a> {
    lifetime: PhantomData<Assignment<'a>>,
}

impl Assignments<'_> {
    pub fn new() -> Self {
        Self {
            lifetime: PhantomData,
        }
    }

    pub fn implies(&self, a: Variable, b: Variable) -> bool {
        let antecedent = self[b].as_ref().map(|assignment| assignment.antecedent());
        true
    }
}

impl<'a> Index<Variable> for Assignments<'a> {
    type Output = Option<Assignment<'a>>;

    #[inline]
    fn index(&self, var: Variable) -> &Self::Output {
        unimplemented!()
    }
}

impl<'a> IndexMut<Variable> for Assignments<'a> {
    #[inline]
    fn index_mut(&mut self, var: Variable) -> &mut Self::Output {
        unimplemented!()
    }
}
