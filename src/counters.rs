use crate::{Assignments, ClauseIdx, Literal, Variable};
use std::{collections::BTreeSet, ops::Index};

pub struct Counters {
    counters: Vec<ClauseIdx>,
    ordered: BTreeSet<(ClauseIdx, Literal)>,
}

impl Counters {
    pub fn new(num_vars: Variable) -> Self {
        // Initialize two counters for each variable (one for each polarity)
        Self {
            counters: vec![0; num_vars * 2 as usize],
            ordered: BTreeSet::new(),
        }
    }

    pub fn next_decision(&self, assignments: &Assignments) -> Option<Literal> {
        self.ordered
            .iter()
            .rev()
            .find(|(_, lit)| assignments.get(lit.var()).is_none())
            .map(|(_, lit)| *lit)
    }

    pub fn increment(&mut self, literal: Literal) {
        let count = &mut self.counters[literal.code() as usize];
        match count.checked_add(1) {
            Some(new) => {
                self.ordered.remove(&(*count, literal));
                self.ordered.insert((new, literal));
                *count = new;
            }
            None => {
                self.counters.iter_mut().for_each(|count| *count /= 2);
                let mut tmp = BTreeSet::new();
                std::mem::swap(&mut self.ordered, &mut tmp);
                tmp = tmp
                    .into_iter()
                    .map(|(count, literal)| (count / 2, literal))
                    .collect();
                std::mem::swap(&mut self.ordered, &mut tmp);
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
