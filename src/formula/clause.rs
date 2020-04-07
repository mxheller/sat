use crate::{partitioned_formula, Assignments, DecisionLevel, Literal, Variable};
use std::collections::BTreeSet;

pub struct Clause {
    literals: BTreeSet<Literal>,
}

impl Clause {
    pub fn new(literals: BTreeSet<Literal>) -> Self {
        Self { literals }
    }

    pub fn literals<'a>(&'a self) -> impl Iterator<Item = Literal> + 'a {
        self.literals.iter().copied()
    }

    pub fn variables<'a>(&'a self) -> impl Iterator<Item = Variable> + 'a {
        self.literals().map(Literal::var)
    }

    pub fn contains(&self, literal: Literal) -> bool {
        self.literals.contains(&literal)
    }

    pub fn resolve(&mut self, other: &partitioned_formula::Clause) {
        match other {
            partitioned_formula::Clause::Binary { a, b } => {
                let (a, b) = (*a, *b);
                if self.contains(!a) {
                    self.literals.remove(&!a);
                    debug_assert!(!self.contains(!b));
                    self.literals.insert(b);
                } else {
                    debug_assert!(self.contains(!b));
                    self.literals.remove(&!b);
                    self.literals.insert(a);
                }
            }
            partitioned_formula::Clause::Many { literals } => {
                let mut found_overlap = false;

                for literal in literals.iter().copied() {
                    debug_assert!(!found_overlap || !self.contains(!literal));
                    if !found_overlap && self.contains(!literal) {
                        self.literals.remove(&!literal);
                        found_overlap = true;
                    } else {
                        self.literals.insert(literal);
                    }
                }
            }
        }
    }

    pub fn literals_assigned_at<'a>(
        &'a self,
        level: DecisionLevel,
        assignments: &'a Assignments,
    ) -> impl Iterator<Item = Literal> + 'a {
        self.literals().filter(move |literal| {
            assignments
                .get(literal.var())
                .map(|assignment| assignment.decision_level() == level)
                .unwrap_or(false)
        })
    }

    pub fn asserting_level(&self, assignments: &Assignments) -> DecisionLevel {
        let mut levels = self
            .variables()
            .map(|var| assignments.get(var).unwrap().decision_level());
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

    pub fn len(&self) -> usize {
        self.literals.len()
    }

    pub fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }
}

impl IntoIterator for Clause {
    type Item = Literal;
    type IntoIter = <BTreeSet<Literal> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.literals.into_iter()
    }
}
