use crate::{Assignments, ClauseIdx, Literal, Watched};
use std::ops::{Index, IndexMut};

pub mod clause;

pub use clause::{Clause, Status};

/// A formula that contains no empty or unit clauses
pub struct TrimmedFormula {
    clauses: Vec<Clause>,
}

impl TrimmedFormula {
    pub fn new(num_clauses: usize) -> Self {
        Self {
            clauses: Vec::with_capacity(num_clauses),
        }
    }

    pub fn add_clause(
        &mut self,
        literals: impl Iterator<Item = Literal> + ExactSizeIterator,
        watched: &mut Watched,
        assignments: &Assignments,
    ) -> (ClauseIdx, Status) {
        let clause = Clause::new(literals);
        let idx = self.clauses.len();
        match &clause {
            Clause::Binary { a, b } => {
                watched[*a].insert(idx);
                watched[*b].insert(idx);
            }
            Clause::Many { literals } => {
                watched[literals[0]].insert(idx);
                watched[literals[1]].insert(idx);
            }
        }
        let idx = self.clauses.len();
        self.clauses.push(clause);
        (idx, self.clauses[idx].update(watched, assignments, idx))
    }
}

impl Index<ClauseIdx> for TrimmedFormula {
    type Output = Clause;

    fn index(&self, idx: ClauseIdx) -> &Self::Output {
        &self.clauses[idx]
    }
}

impl IndexMut<ClauseIdx> for TrimmedFormula {
    fn index_mut(&mut self, idx: ClauseIdx) -> &mut Self::Output {
        &mut self.clauses[idx]
    }
}
