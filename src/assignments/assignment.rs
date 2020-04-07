use crate::{ClauseIdx, DecisionLevel, Sign};

#[derive(Clone, Debug)]
pub struct Assignment {
    sign: Sign,
    antecedent: Option<ClauseIdx>,
    decision_level: DecisionLevel,
}

impl Assignment {
    pub fn decided(sign: Sign, decision_level: DecisionLevel) -> Self {
        Self {
            sign,
            antecedent: None,
            decision_level,
        }
    }

    pub fn implied(sign: Sign, antecedent: ClauseIdx, decision_level: DecisionLevel) -> Self {
        Self {
            sign,
            antecedent: Some(antecedent),
            decision_level,
        }
    }

    pub fn sign(&self) -> Sign {
        self.sign
    }

    pub fn antecedent(&self) -> Option<ClauseIdx> {
        self.antecedent
    }

    pub fn decision_level(&self) -> DecisionLevel {
        self.decision_level
    }
}
