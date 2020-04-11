use crate::{Literal, Variable};

#[derive(Debug, PartialEq)]
pub struct Clause {
    literals: Vec<Literal>,
}

impl Clause {
    pub fn literals<'a>(&'a self) -> impl Iterator<Item = Literal> + ExactSizeIterator + 'a {
        self.literals.iter().copied()
    }

    pub fn variables<'a>(&'a self) -> impl Iterator<Item = Variable> + ExactSizeIterator + 'a {
        self.literals().map(Literal::var)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Literal> {
        self.literals.iter_mut()
    }
}

impl std::str::FromStr for Clause {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let literals = s
            .split_whitespace()
            .filter(|s| *s != "0")
            .map(|s| s.parse::<isize>().map(Literal::from))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { literals })
    }
}

impl From<Vec<Literal>> for Clause {
    fn from(literals: Vec<Literal>) -> Self {
        Self { literals }
    }
}

impl Into<Vec<Literal>> for Clause {
    fn into(self) -> Vec<Literal> {
        self.literals
    }
}
