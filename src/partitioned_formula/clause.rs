use crate::{formula, Assignments, ClauseIdx, Evaluate, Literal, Variable, Watched};

pub enum Clause {
    Binary { a: Literal, b: Literal },
    Many { literals: Vec<Literal> },
}

pub enum Status {
    Ok,
    Conflict(formula::Clause),
    Implied(Literal),
}

impl Clause {
    pub fn new(literals: Vec<Literal>) -> Clause {
        assert!(literals.len() > 1);
        match &literals.as_slice() {
            [a, b] => Self::Binary { a: *a, b: *b },
            _ => Self::Many { literals },
        }
    }

    pub fn max_variable(&self) -> Variable {
        match self {
            Self::Binary { a, b } => std::cmp::max(a.var(), b.var()),
            Self::Many { ref literals } => {
                literals.iter().copied().map(Literal::var).max().unwrap()
            }
        }
    }

    /// Restores the 2-Watched Literal invariant and
    /// produces a new implied literal if one exists
    pub fn update(
        &mut self,
        watched: &mut Watched,
        assignments: &Assignments,
        clause_idx: ClauseIdx,
    ) -> Status {
        match self {
            Self::Binary { a, b } => match (a.evaluate(assignments), b.evaluate(assignments)) {
                (Some(true), _) | (_, Some(true)) => Status::Ok,
                (None, Some(false)) => Status::Implied(*a),
                (Some(false), None) => Status::Implied(*b),
                (Some(false), Some(false)) => Status::Conflict(self.get_literals()),
                (None, None) => panic!("Neither watched literal was affected"),
            },
            Self::Many { ref mut literals } => {
                // Determines the value of a literal in the current assignment
                let value = |idx: usize| literals[idx].evaluate(assignments);

                let mut watch = |literals: &mut Vec<Literal>, idx, slot| {
                    if idx != slot {
                        watched[literals[slot]].remove(&clause_idx);
                        watched[literals[idx]].insert(clause_idx);
                        literals.swap(idx, slot);
                    }
                };

                // Indices of literals that do not evaluate to false
                let mut not_false =
                    (0..literals.len()).filter(|idx| !matches!(value(*idx), Some(false)));

                match (value(0), value(1)) {
                    // Both watched literals are still unassigned or one is satisfied
                    (None, None) | (Some(true), _) | (_, Some(true)) => Status::Ok,

                    // At least one of the watched literals is false
                    _ => match (not_false.next(), not_false.next()) {
                        // There are no non-false literals--conflict
                        (None, None) => Status::Conflict(self.get_literals()),

                        // There is only one non-false literal, so it must be true
                        (Some(a), None) => {
                            watch(literals, a, 0);
                            Status::Implied(literals[0])
                        }

                        // There are multiple non-false literals--watch them
                        (Some(a), Some(b)) => {
                            watch(literals, a, 0);
                            watch(literals, b, 1);
                            Status::Ok
                        }

                        // Iterators don't work like this
                        (None, Some(_)) => unreachable!(),
                    },
                }
            }
        }
    }

    pub fn get_literals(&self) -> formula::Clause {
        formula::Clause::new(match self {
            Self::Binary { a, b } => [*a, *b].iter().copied().collect(),
            Self::Many { literals } => literals.iter().copied().collect(),
        })
    }
}

impl Evaluate for Clause {
    fn evaluate(&self, assignments: &Assignments) -> Option<bool> {
        match self {
            Self::Binary { a, b } => a.evaluate(assignments).or_else(|| b.evaluate(assignments)),
            Self::Many { literals } => literals
                .iter()
                .map(|literal| literal.evaluate(assignments))
                .collect::<Option<Vec<_>>>()
                .map(|truths| truths.iter().any(|x| *x)),
        }
    }
}
