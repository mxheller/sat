use crate::{
    assignments::Assignments, formula::Clause, sign::Sign, ClauseIdx, DecisionLevel, Variable,
};

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Literal {}

impl Literal {
    pub fn new(var: Variable, sign: Sign) -> Self {
        unimplemented!()
    }

    pub fn var(&self) -> Variable {
        unimplemented!();
    }

    pub fn sign(&self) -> Sign {
        unimplemented!();
    }
}

impl std::ops::Not for Literal {
    type Output = Literal;

    #[inline]
    fn not(self) -> Self::Output {
        unimplemented!()
    }
}
