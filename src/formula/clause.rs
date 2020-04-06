use crate::{
    assignments::Assignments,
    formula::Literal,
    sign::Sign::{Negative, Positive},
    DecisionLevel, Variable,
};
use std::collections::BTreeSet;

#[derive(Clone)]
pub struct Clause {
    literals: BTreeSet<Literal>,
}

impl Clause {
    pub fn literals(&self) -> impl Iterator<Item = &Literal> {
        unimplemented!();
        [].iter()
    }

    pub fn variables<'a>(&'a self) -> impl Iterator<Item = Variable> + 'a {
        self.literals().map(Literal::var)
    }

    pub fn contains(&self, literal: &Literal) -> bool {
        self.literals.contains(literal)
    }

    pub fn contains_pos(&self, var: Variable) -> bool {
        unimplemented!()
    }

    pub fn contains_neg(&self, var: Variable) -> bool {
        unimplemented!()
    }

    pub fn resolve(&mut self, other: &Clause) {
        let mut found_overlap = false;

        for literal in other.literals() {
            debug_assert!(!(found_overlap && self.contains(&!*literal)));
            if !found_overlap && self.contains(&!*literal) {
                self.literals.remove(&!*literal);
                found_overlap = true;
            } else {
                self.literals.insert(*literal);
            }
        }
    }

    pub fn literals_assigned_at<'a>(
        &'a self,
        level: DecisionLevel,
        assignments: &'a Assignments,
    ) -> impl Iterator<Item = &Literal> + 'a {
        self.literals().filter(move |literal| {
            assignments
                .get(literal.var(), level)
                .as_ref()
                .map(|assignment| assignment.decision_level() == level)
                .unwrap_or(false)
        })
    }

    pub fn asserting_level(
        &self,
        assignments: &Assignments,
        current_level: DecisionLevel,
    ) -> DecisionLevel {
        let mut levels = self.variables().map(|var| {
            assignments
                .get(var, current_level)
                .unwrap()
                .decision_level()
        });
        match (levels.next(), levels.next()) {
            (Some(first), Some(second)) => {
                let (mut highest, mut second_highest) =
                    (std::cmp::max(first, second), std::cmp::min(first, second));
                for level in levels {
                    if level > highest {
                        second_highest = highest;
                        highest = level;
                    } else if level > second_highest {
                        second_highest = level
                    }
                }
                second_highest
            }
            _ => 0,
        }
    }
}
