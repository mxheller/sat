use crate::{Assignments, ClauseIdx, Evaluate, Literal, Watched};

#[derive(Debug)]
pub enum Clause {
    Binary { a: Literal, b: Literal },
    Many { literals: Vec<Literal> },
}

#[must_use]
#[derive(Debug, PartialEq)]
pub enum Status {
    Ok,
    Conflict,
    Implied(Literal),
}

impl Clause {
    pub fn new(
        mut literals: impl Iterator<Item = Literal> + ExactSizeIterator,
    ) -> Result<Self, String> {
        match literals.len() {
            0 | 1 => Err("TrimmedFormula should only contain clauses with len > 1".to_string()),
            2 => Ok(Self::Binary {
                a: literals.next().unwrap(),
                b: literals.next().unwrap(),
            }),
            _ => Ok(Self::Many {
                literals: literals.collect(),
            }),
        }
    }

    /// Restores the 2-Watched Literal invariant and produces a new implied literal if one exists
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
                (Some(false), Some(false)) => Status::Conflict,
                (None, None) => Status::Ok,
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
                        (None, None) => Status::Conflict,

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
}

#[test]
fn new_clause() -> Result<(), String> {
    assert!(Clause::new(std::iter::empty()).is_err());
    assert!(Clause::new(std::iter::once(Literal::new(5, false))).is_err());

    let (l1, l2, l3, l4) = (
        Literal::new(1, true),
        Literal::new(2, true),
        Literal::new(3, true),
        Literal::new(4, false),
    );

    let lits = vec![l2, l4, l1, l3];

    let binary = Clause::new(lits[..2].iter().copied())?;
    assert!(matches!(binary, Clause::Binary{a, b} if a==l2 && b==l4));

    let ternary = Clause::new(lits[..3].iter().copied())?;
    assert!(matches!(ternary, Clause::Many{literals} if literals == &lits[..3]));

    let all = Clause::new(lits.iter().copied())?;
    assert!(matches!(all, Clause::Many{literals} if literals == lits));

    Ok(())
}

#[test]
fn update_binary() -> Result<(), String> {
    use crate::sign::Sign::{Negative, Positive};

    let (l0, l1) = (Literal::new(0, true), Literal::new(1, true));
    let watched = &mut Watched::new(2);
    let clause = &mut Clause::new([l0, l1].iter().copied())?;

    assert_eq!(
        clause.update(watched, &Assignments::new_with(vec![None, None]), 0),
        Status::Ok
    );

    assert_eq!(
        clause.update(
            watched,
            &Assignments::new_with(vec![Some(Negative), None]),
            0
        ),
        Status::Implied(l1)
    );

    assert_eq!(
        clause.update(
            watched,
            &Assignments::new_with(vec![Some(Positive), None]),
            0
        ),
        Status::Ok
    );

    assert_eq!(
        clause.update(
            watched,
            &Assignments::new_with(vec![None, Some(Positive)]),
            0
        ),
        Status::Ok
    );

    assert_eq!(
        clause.update(
            watched,
            &Assignments::new_with(vec![Some(Positive), Some(Positive)]),
            0
        ),
        Status::Ok
    );

    assert_eq!(
        clause.update(
            watched,
            &Assignments::new_with(vec![None, Some(Negative)]),
            0
        ),
        Status::Implied(l0)
    );

    // Both false
    assert_eq!(
        clause.update(
            watched,
            &Assignments::new_with(vec![Some(Negative), Some(Negative)]),
            0
        ),
        Status::Conflict
    );

    Ok(())
}

#[test]
fn update_ternary() -> Result<(), String> {
    use crate::sign::Sign::{Negative, Positive};

    let (l0, l1, l2) = (
        Literal::new(0, true),
        Literal::new(1, true),
        Literal::new(2, true),
    );
    let watched = &mut Watched::new(3);
    let clause = &mut Clause::new([l0, !l1, l2].iter().copied())?;

    watched[l0].insert(0);
    watched[!l1].insert(0);

    assert_eq!(
        clause.update(watched, &Assignments::new_with(vec![None, None, None]), 0),
        Status::Ok
    );

    assert_eq!(
        clause.update(
            watched,
            &Assignments::new_with(vec![Some(Negative), None, None]),
            0
        ),
        Status::Ok
    );
    // -> l1, l2 watched
    assert!(watched[l0].is_empty());
    assert!(watched[!l1].contains(&0));
    assert!(watched[l2].contains(&0));
    assert!(watched[!l0].is_empty());
    assert!(watched[l1].is_empty());
    assert!(watched[!l2].is_empty());

    assert_eq!(
        clause.update(watched, &Assignments::new_with(vec![None, None, None]), 0),
        Status::Ok
    );
    // nothing should have changed
    assert!(watched[l0].is_empty());
    assert!(watched[!l1].contains(&0));
    assert!(watched[l2].contains(&0));
    assert!(watched[!l0].is_empty());
    assert!(watched[l1].is_empty());
    assert!(watched[!l2].is_empty());

    // set !l1, l2 to false
    assert_eq!(
        clause.update(
            watched,
            &Assignments::new_with(vec![None, Some(Positive), Some(Negative)]),
            0
        ),
        Status::Implied(l0)
    );
    // l0 and one of !l1, l2 should be watched
    assert!(watched[l0].contains(&0));
    assert!(watched[!l1].contains(&0) ^ watched[l2].contains(&0));
    assert!(watched[!l0].is_empty());
    assert!(watched[l1].is_empty());
    assert!(watched[!l2].is_empty());

    // set l2 to be true
    assert_eq!(
        clause.update(
            watched,
            &Assignments::new_with(vec![None, Some(Positive), Some(Positive)]),
            0
        ),
        Status::Ok
    );
    // -> l0, l2 watched
    assert!(watched[l0].contains(&0));
    assert!(watched[!l1].is_empty());
    assert!(watched[l2].contains(&0));
    assert!(watched[!l0].is_empty());
    assert!(watched[l1].is_empty());
    assert!(watched[!l2].is_empty());

    Ok(())
}
