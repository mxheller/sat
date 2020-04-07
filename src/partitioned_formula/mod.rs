use crate::{Assignments, ClauseIdx, Evaluate, Literal, Variable};
use std::ops::{Index, IndexMut};

pub mod clause;

pub use clause::Clause;

pub struct PartitionedFormula {
    contains_empty_clause: bool,
    unit_clauses: Vec<Literal>,
    remaining_clauses: Vec<Clause>,
}

impl PartitionedFormula {
    pub fn num_variables(&self) -> Variable {
        self.max_variable().map(|var| var + 1).unwrap_or(0)
    }

    pub fn max_variable(&self) -> Option<Variable> {
        let unit_vars = self.unit_clauses.iter().copied().map(Literal::var);
        let rest_vars = self.remaining_clauses.iter().map(Clause::max_variable);
        match (unit_vars.max(), rest_vars.max()) {
            (None, None) => None,
            (Some(x), None) | (None, Some(x)) => Some(x),
            (Some(a), Some(b)) => Some(std::cmp::max(a, b)),
        }
    }

    pub fn add_clause(&mut self, literals: impl IntoIterator<Item = Literal>) {
        // TODO: make cleaner
        let literals = literals.into_iter().collect::<Vec<_>>();
        match literals.len() {
            0 => self.contains_empty_clause = true,
            1 => self.unit_clauses.push(literals[0]),
            _ => self.remaining_clauses.push(Clause::new(literals)),
        }
    }

    pub fn take_units(&mut self) -> Vec<Literal> {
        let mut units = Vec::new();
        std::mem::swap(&mut units, &mut self.unit_clauses);
        units
    }
}

impl Evaluate for PartitionedFormula {
    fn evaluate(&self, assignments: &Assignments) -> Option<bool> {
        if self.contains_empty_clause {
            Some(false)
        } else {
            let units = self
                .unit_clauses
                .iter()
                .map(|literal| literal.evaluate(assignments));
            self.remaining_clauses
                .iter()
                .map(|clause| clause.evaluate(assignments))
                .chain(units)
                .collect::<Option<Vec<_>>>()
                .map(|truths| truths.iter().all(|x| *x))
        }
    }
}

impl Index<ClauseIdx> for PartitionedFormula {
    type Output = Clause;

    fn index(&self, idx: ClauseIdx) -> &Self::Output {
        &self.remaining_clauses[idx]
    }
}

impl IndexMut<ClauseIdx> for PartitionedFormula {
    fn index_mut(&mut self, idx: ClauseIdx) -> &mut Self::Output {
        &mut self.remaining_clauses[idx]
    }
}
