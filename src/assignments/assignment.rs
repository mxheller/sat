use crate::{assignments::Assignments, formula::Clause, sign::Sign, DecisionLevel, Variable};
use std::cell::{Ref, RefCell};

pub type Antecedent<'a> = &'a RefCell<Clause>;

pub struct Assignment<'a> {
    sign: Sign,
    antecedent: Option<Antecedent<'a>>,
    decision_level: DecisionLevel,
}

impl<'a> Assignment<'a> {
    pub fn decided(sign: Sign, decision_level: DecisionLevel) -> Self {
        Self {
            sign,
            antecedent: None,
            decision_level,
        }
    }

    pub fn implied(
        var: Variable,
        sign: Sign,
        antecedent: Antecedent<'a>,
        assignments: &Assignments,
    ) -> Self {
        let decision_level = antecedent
            .borrow()
            .variables()
            .filter_map(|var| assignments[var].as_ref())
            .map(|assignment| assignment.decision_level)
            .max()
            .unwrap_or(0);

        Self {
            sign,
            antecedent: Some(antecedent),
            decision_level,
        }
    }

    pub fn sign(&self) -> Sign {
        self.sign
    }

    pub fn antecedent(&self) -> Option<Ref<Clause>> {
        self.antecedent.map(|antecedent| antecedent.borrow())
    }

    pub fn decision_level(&self) -> DecisionLevel {
        self.decision_level
    }
}
