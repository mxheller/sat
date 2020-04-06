use crate::{assignments::Assignments, formula::Clause, sign::Sign, DecisionLevel, Variable};
use std::cell::Ref;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Literal {}

impl Literal {
    pub fn var(&self) -> Variable {
        unimplemented!();
    }

    pub fn sign(&self) -> Sign {
        unimplemented!();
    }

    pub fn implied_in_at_level<'a, 'b>(
        &'a self,
        clause: impl std::ops::Deref<Target = Clause>,
        level: DecisionLevel,
        assignments: &'b Assignments,
    ) -> Option<Ref<'b, Clause>> {
        assignments[self.var()].as_ref().and_then(|ref assignment| {
            if assignment.decision_level() == level && clause.contains(self) {
                assignment.antecedent()
            } else {
                None
            }
        })
    }
}

impl std::ops::Not for Literal {
    type Output = Literal;

    #[inline]
    fn not(self) -> Self::Output {
        unimplemented!()
    }
}
