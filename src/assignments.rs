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
