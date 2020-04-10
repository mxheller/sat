use crate::{Assignments, ClauseIdx, Literal, Variable};
use std::ops::Index;

pub type Count = ClauseIdx;

pub struct Counters {
    /// Count of each literal (where each literal's code corresponds to its index)
    counts: Vec<Count>,
    /// Position of each literal in the ordered vec
    positions: Vec<usize>,
    /// Literals sorted in increasing order by count
    ordered: Vec<Literal>,
}

impl Counters {
    pub fn new(num_vars: Variable) -> Self {
        // Initialize two counters for each variable (one for each polarity)
        let num_literals = num_vars * 2 as usize;
        Self {
            counts: vec![0; num_literals],
            positions: (0..num_literals).collect(),
            ordered: (0..num_literals).map(Literal::from_code).collect(),
        }
    }

    pub fn next_decision(&self, assignments: &Assignments) -> Option<Literal> {
        self.ordered
            .iter()
            .rev()
            .find(|lit| assignments.get(lit.var()).is_none())
            .copied()
    }

    pub fn increment(&mut self, literal: Literal) {
        // Get index of literal
        let idx = literal.code() as usize;
        // Find current count and increment
        let count = self.counts[idx];
        self.counts[idx] += 1;
        // Find the last literal in same chunk of the sorted vec that has the same count
        let target = self.ordered[self.positions[idx] + 1..]
            .iter()
            .take_while(|literal| self.counts[literal.code()] == count)
            .last()
            .copied();

        if let Some(target) = target {
            // Swap with the target
            // This leaves the target in the same chunk as it was before,
            // but leaves our new literal in the right position for its count
            let target_idx = target.code();
            self.ordered
                .swap(self.positions[idx], self.positions[target_idx]);
            self.positions.swap(idx, target_idx);
            debug_assert_ne!(target, literal);
            debug_assert_eq!(self.ordered[self.positions[target_idx]], target);
            debug_assert_eq!(self.ordered[self.positions[idx]], literal);
        } else {
            // There are no other literals with the same count, so incrementing
            // will preserve the order
        }

        debug_assert!(self
            .ordered
            .is_sorted_by_key(|literal| self.counts[literal.code()]));
    }
}

impl Index<Literal> for Counters {
    type Output = ClauseIdx;

    #[inline]
    fn index(&self, literal: Literal) -> &Self::Output {
        &self.counts[literal.code() as usize]
    }
}
