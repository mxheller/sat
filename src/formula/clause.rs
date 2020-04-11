use crate::{Assignment, Assignments, DecisionLevel, Literal, Variable};
use std::collections::BTreeSet;

#[derive(Debug, PartialEq)]
pub struct Clause {
    literals: BTreeSet<Literal>,
}

impl Clause {
    pub fn empty() -> Self {
        Self {
            literals: BTreeSet::new(),
        }
    }

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

    pub fn implied_at<'a>(
        &'a self,
        level: DecisionLevel,
        assignments: &'a Assignments,
    ) -> impl Iterator<Item = (Literal, &Assignment)> + 'a {
        self.implied(assignments)
            .filter(move |(_, assignment)| assignment.decision_level() == level)
    }

    pub fn implied<'a>(
        &'a self,
        assignments: &'a Assignments,
    ) -> impl Iterator<Item = (Literal, &Assignment)> + 'a {
        self.literals().filter_map(move |literal| {
            assignments
                .get(literal.var())
                .map(|assignment| (literal, assignment))
        })
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

impl From<Vec<Literal>> for Clause {
    fn from(literals: Vec<Literal>) -> Self {
        Self {
            literals: literals.into_iter().collect(),
        }
    }
}
