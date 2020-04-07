use crate::{ClauseIdx, Variable};
use std::ops::{Index, IndexMut};

pub mod clause;
pub mod literal;

pub use clause::{Clause, Literals};
pub use literal::Literal;

pub struct Formula {
    contains_empty_clause: bool,
    unit_clauses: Vec<Literal>,
    remaining_clauses: Vec<Clause>,
}

impl Formula {
    pub fn num_variables(&self) -> Variable {
        self.max_variable().map(|var| var + 1).unwrap_or(0)
    }

    pub fn max_variable(&self) -> Option<Variable> {
        let unit_vars = self.unit_clauses.iter().map(Literal::var);
        let rest_vars = self.remaining_clauses.iter().map(Clause::max_variable);
        match (unit_vars.max(), rest_vars.max()) {
            (None, None) => None,
            (Some(x), None) | (None, Some(x)) => Some(x),
            (Some(a), Some(b)) => Some(std::cmp::max(a, b)),
        }
    }

    pub fn add_clause(&mut self, literals: Literals) {
        match literals.len() {
            0 => self.contains_empty_clause = true,
            1 => self.unit_clauses.push(*literals.literals().next().unwrap()),
            _ => self.remaining_clauses.push(Clause::new(literals.literals)),
        }
    }

    pub fn take_units(&mut self) -> Vec<Literal> {
        let mut units = Vec::new();
        std::mem::swap(&mut units, &mut self.unit_clauses);
        units
    }
}

impl Index<ClauseIdx> for Formula {
    type Output = Clause;

    fn index(&self, idx: ClauseIdx) -> &Self::Output {
        &self.remaining_clauses[idx]
    }
}

impl IndexMut<ClauseIdx> for Formula {
    fn index_mut(&mut self, idx: ClauseIdx) -> &mut Self::Output {
        &mut self.remaining_clauses[idx]
    }
}
