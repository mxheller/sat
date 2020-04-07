use crate::{assignments::Assignments, sign::Sign, ClauseIdx, Evaluate, Variable};

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

impl Evaluate for Literal {
    fn evaluate(&self, assignments: &Assignments) -> Option<bool> {
        assignments
            .get(self.var())
            .map(|assignment| assignment.sign() == self.sign())
    }
}

impl std::ops::Not for Literal {
    type Output = Literal;

    #[inline]
    fn not(self) -> Self::Output {
        unimplemented!()
    }
}
