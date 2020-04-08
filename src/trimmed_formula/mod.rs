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
    ) -> Result<(ClauseIdx, Status), String> {
        let clause = Clause::new(literals)?;
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
        Ok((idx, self.clauses[idx].update(watched, assignments, idx)))
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

#[test]
fn add_clause() -> Result<(), String> {
    use crate::sign::Sign::Positive;

    let mut formula = TrimmedFormula::new(2);
    let watched = &mut Watched::new(2);
    let assignments = &mut Assignments::new(2);

    let (l0, l1) = (Literal::new(0, Positive), Literal::new(1, Positive));

    assert_eq!(
        formula.add_clause([l0, l1].iter().copied(), watched, assignments)?,
        (0, Status::Ok)
    );
    assert!(watched[l0].contains(&0));
    assert!(watched[l1].contains(&0));
    assert!(watched[!l0].is_empty());
    assert!(watched[!l1].is_empty());
    assert_eq!(formula.clauses.len(), 1);

    assignments.set_unchecked(0, Positive);
    assert_eq!(
        formula.add_clause([!l0, l1].iter().copied(), watched, assignments)?,
        (1, Status::Implied(l1))
    );
    assert!(watched[l0].contains(&0));
    assert_eq!(watched[l0].len(), 1);
    assert!(watched[l1].contains(&0));
    assert!(watched[l1].contains(&1));
    assert_eq!(watched[l1].len(), 2);
    assert!(watched[!l0].contains(&1));
    assert_eq!(watched[!l0].len(), 1);
    assert!(watched[!l1].is_empty());
    assert_eq!(formula.clauses.len(), 2);

    Ok(())
}
