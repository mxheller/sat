use crate::{formula::Literal, sign::Sign, ClauseIdx, Variable};
use std::{
    collections::BTreeSet,
    ops::{Index, IndexMut},
};

pub struct Watched {
    watched: Vec<Clauses>,
}

#[derive(Clone, Default)]
pub struct Clauses {
    positive: BTreeSet<ClauseIdx>,
    negative: BTreeSet<ClauseIdx>,
}

impl Watched {
    pub fn new(num_vars: Variable) -> Self {
        Self {
            watched: vec![Default::default(); num_vars as usize],
        }
    }
}

impl Index<Literal> for Watched {
    type Output = BTreeSet<ClauseIdx>;

    #[inline]
    fn index(&self, literal: Literal) -> &Self::Output {
        &self.watched[literal.var()][literal.sign()]
    }
}

impl IndexMut<Literal> for Watched {
    #[inline]
    fn index_mut(&mut self, literal: Literal) -> &mut Self::Output {
        &mut self.watched[literal.var()][literal.sign()]
    }
}

impl Index<Sign> for Clauses {
    type Output = BTreeSet<ClauseIdx>;

    #[inline]
    fn index(&self, sign: Sign) -> &Self::Output {
        match sign {
            Sign::Positive => &self.positive,
            Sign::Negative => &self.negative,
        }
    }
}

impl IndexMut<Sign> for Clauses {
    #[inline]
    fn index_mut(&mut self, sign: Sign) -> &mut Self::Output {
        match sign {
            Sign::Positive => &mut self.positive,
            Sign::Negative => &mut self.negative,
        }
    }
}
