use crate::{trimmed_formula, Assignments, DecisionLevel, Evaluate, Literal, Variable};
use std::collections::BTreeSet;

pub struct Clause {
    literals: BTreeSet<Literal>,
}

impl Clause {
    pub fn new(literals: BTreeSet<Literal>) -> Self {
        Self { literals }
    }

    pub fn literals<'a>(&'a self) -> impl Iterator<Item = Literal> + ExactSizeIterator + 'a {
        self.literals.iter().copied()
    }

    pub fn variables<'a>(&'a self) -> impl Iterator<Item = Variable> + ExactSizeIterator + 'a {
        self.literals().map(Literal::var)
    }

    pub fn contains(&self, literal: Literal) -> bool {
        self.literals.contains(&literal)
    }

    pub fn resolve(&mut self, other: &trimmed_formula::Clause) {
        match other {
            trimmed_formula::Clause::Binary { a, b } => {
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
            trimmed_formula::Clause::Many { literals } => {
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

    /// Returns a decision level from which the clause can still be satisfied
    pub fn satisfiable_level(&self, assignments: &Assignments) -> Option<DecisionLevel> {
        // FIXME:

        // Make sure all literals are actually unsatisfied
        debug_assert!(self
            .literals()
            .all(|literal| matches!(literal.evaluate(assignments), Some(false))));

        // Get decision levels of each literal's variable
        let mut levels = self
            .variables()
            .map(|var| assignments.get(var).unwrap().decision_level());

        // It is impossible to fix an unsatisfied clause if it is unit or empty
        if levels.len() < 2 {
            return None;
        }

        let (first, second) = (levels.next().unwrap(), levels.next().unwrap());
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
        Some(second_highest)
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
