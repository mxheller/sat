use crate::{trimmed_formula, Assignment, Assignments, DecisionLevel, Evaluate, Literal, Variable};
use std::collections::BTreeSet;

#[derive(Debug)]
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

    pub fn resolve(&mut self, literal: Literal, other: &trimmed_formula::Clause) {
        assert!(self.literals.remove(&literal));

        match other {
            trimmed_formula::Clause::Binary { a, b } if *a == !literal => {
                self.literals.insert(*b);
            }
            trimmed_formula::Clause::Binary { a, b } if *b == !literal => {
                self.literals.insert(*a);
            }
            trimmed_formula::Clause::Many { literals } => {
                self.literals
                    .extend(literals.iter().copied().filter(|x| *x != !literal));
            }
            _ => panic!("'antecedent' clause wasn't actually antecedent"),
        }
    }

    pub fn assignments_at<'a>(
        &'a self,
        level: DecisionLevel,
        assignments: &'a Assignments,
    ) -> impl Iterator<Item = (Literal, &Assignment)> + 'a {
        self.assignments(assignments)
            .filter(move |(_, assignment)| assignment.decision_level() == level)
    }

    pub fn assignments<'a>(
        &'a self,
        assignments: &'a Assignments,
    ) -> impl Iterator<Item = (Literal, &Assignment)> + 'a {
        self.literals().filter_map(move |literal| {
            assignments
                .get(literal.var())
                .map(|assignment| (literal, assignment))
        })
    }

    /// Returns a decision level from which the clause can still be satisfied
    pub fn backtrack_level(
        &self,
        conflict_level: DecisionLevel,
        assignments: &Assignments,
    ) -> Option<DecisionLevel> {
        // Make sure all literals are actually unsatisfied
        debug_assert!(self
            .literals()
            .all(|literal| matches!(literal.evaluate(assignments), Some(false))));

        // Ensure there is a single literal assigned at the conflict level
        debug_assert_eq!(
            self.variables()
                .filter(|var| assignments.get(*var).unwrap().decision_level() == conflict_level)
                .count(),
            1,
            "There should be exactly one literal assigned at the conflict level in the clause to be learned"
        );

        // Return the maximum level below the conflict level
        self.variables()
            .map(|var| assignments.get(var).unwrap().decision_level())
            .filter(|level| *level != conflict_level)
            .max()
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

impl std::str::FromStr for Clause {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let literals = s
            .split_whitespace()
            .filter(|s| *s != "0")
            .map(|s| s.parse::<isize>().map(Literal::from))
            .collect::<Result<BTreeSet<_>, _>>()?;
        Ok(Self { literals })
    }
}

impl From<Literal> for Clause {
    fn from(literal: Literal) -> Self {
        let mut literals = BTreeSet::new();
        literals.insert(literal);
        Self { literals }
    }
}
