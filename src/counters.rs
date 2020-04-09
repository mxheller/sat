use crate::{Assignments, ClauseIdx, Literal, Variable};
use std::ops::Index;

pub struct Counters {
    counters: Vec<ClauseIdx>,
}

impl Counters {
    pub fn new(num_vars: Variable) -> Self {
        // Initialize two counters for each variable (one for each polarity)
        Self {
            counters: vec![0; num_vars * 2 as usize],
        }
    }

    pub fn next_decision(&self, assignments: &Assignments) -> Option<Literal> {
        self.counters
            .iter()
            .enumerate()
            .map(|(code, count)| (Literal::from_code(code), count))
            .filter(|(lit, _)| assignments.get(lit.var()).is_none())
            .max_by_key(|pair| pair.1)
            .map(|(lit, _)| lit)
    }

    pub fn increment(&mut self, literal: Literal) {
        let count = &mut self.counters[literal.code() as usize];
        match count.checked_add(1) {
            Some(new) => *count = new,
            None => {
                self.counters.iter_mut().for_each(|count| *count /= 2);
                self.increment(literal);
            }
        }
    }
}

impl Index<Literal> for Counters {
    type Output = ClauseIdx;

    #[inline]
    fn index(&self, literal: Literal) -> &Self::Output {
        &self.counters[literal.code() as usize]
    }
}
