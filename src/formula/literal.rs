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

    pub fn implied_in_at_level(
        &self,
        clause: &Clause,
        level: DecisionLevel,
        assignments: &Assignments,
    ) -> Option<ClauseIdx> {
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
