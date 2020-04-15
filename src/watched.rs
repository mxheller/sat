use crate::{ClauseIdx, Literal, Variable};
use std::ops::{Index, IndexMut};

pub struct Watched {
    watched: Vec<Vec<ClauseIdx>>,
}

impl Watched {
    pub fn new(num_vars: Variable) -> Self {
        Self {
            watched: vec![Vec::new(); num_vars * 2],
        }
    }
}

impl Index<Literal> for Watched {
    type Output = Vec<ClauseIdx>;

    #[inline]
    fn index(&self, literal: Literal) -> &Self::Output {
        &self.watched[literal.code()]
    }
}

impl IndexMut<Literal> for Watched {
    #[inline]
    fn index_mut(&mut self, literal: Literal) -> &mut Self::Output {
        &mut self.watched[literal.code()]
    }
}
