use crate::{
    assignments::Assignments, evaluate::Evaluate, formula::Literal, watched::Watched, ClauseIdx,
    DecisionLevel, Variable,
};

#[derive(Clone)]
pub enum Clause {
    Binary { a: Literal, b: Literal },
    Many { literals: Vec<Literal> },
}

pub enum ClauseUpdateResult {
    Ok,
    Conflict(Literals),
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

    /// Restores the 2-Watched Literal invariant and
    /// produces a new implied literal if one exists
    pub fn update(
        &mut self,
        watched: &mut Watched,
        assignments: &Assignments,
        clause_idx: ClauseIdx,
    ) -> ClauseUpdateResult {
        match self {
            Self::Binary { a, b } => match (a.evaluate(assignments), b.evaluate(assignments)) {
                (Some(true), _) | (_, Some(true)) => ClauseUpdateResult::Ok,
                (None, Some(false)) => ClauseUpdateResult::Implied(*a),
                (Some(false), None) => ClauseUpdateResult::Implied(*b),
                (Some(false), Some(false)) => ClauseUpdateResult::Conflict(self.literals()),
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
                    (None, None) | (Some(true), _) | (_, Some(true)) => ClauseUpdateResult::Ok,

                    // At least one of the watched literals is false
                    _ => match (not_false.next(), not_false.next()) {
                        // There are no non-false literals--conflict
                        (None, None) => ClauseUpdateResult::Conflict(self.literals()),

                        // There is only one non-false literal, so it must be true
                        (Some(a), None) => {
                            watch(literals, a, 0);
                            ClauseUpdateResult::Implied(literals[0])
                        }

                        // There are multiple non-false literals--watch them
                        (Some(a), Some(b)) => {
                            watch(literals, a, 0);
                            watch(literals, b, 1);
                            ClauseUpdateResult::Ok
                        }

                        // Iterators don't work like this
                        (None, Some(_)) => unreachable!(),
                    },
                }
            }
        }
    }

    pub fn literals(&self) -> Literals {
        Literals {
            literals: match self {
                Self::Binary { a, b } => vec![*a, *b],
                Self::Many { literals } => literals.clone(),
            },
        }
    }
}

pub struct Literals {
    pub literals: Vec<Literal>,
}

impl Literals {
    pub fn literals(&self) -> impl Iterator<Item = &Literal> {
        self.literals.iter()
    }

    pub fn variables<'a>(&'a self) -> impl Iterator<Item = Variable> + 'a {
        self.literals().map(Literal::var)
    }

    pub fn contains(&self, literal: &Literal) -> bool {
        unimplemented!();
        self.literals.contains(literal)
    }

    pub fn contains_pos(&self, var: Variable) -> bool {
        unimplemented!()
    }

    pub fn contains_neg(&self, var: Variable) -> bool {
        unimplemented!()
    }

    pub fn resolve(&mut self, other: &Clause) {
        unimplemented!();
        // let mut found_overlap = false;

        // for literal in other.literals() {
        //     debug_assert!(!(found_overlap && self.contains(&!*literal)));
        //     if !found_overlap && self.contains(&!*literal) {
        //         self.literals.remove(&!*literal);
        //         found_overlap = true;
        //     } else {
        // self.literals.insert(*literal);
        // }
        // }
    }

    pub fn literals_assigned_at<'a>(
        &'a self,
        level: DecisionLevel,
        assignments: &'a Assignments,
    ) -> impl Iterator<Item = &Literal> + 'a {
        self.literals().filter(move |literal| {
            assignments
                .get(literal.var())
                .map(|assignment| assignment.decision_level() == level)
                .unwrap_or(false)
        })
    }

    pub fn asserting_level(&self, assignments: &Assignments) -> DecisionLevel {
        let mut levels = self
            .variables()
            .map(|var| assignments.get(var).unwrap().decision_level());
        match (levels.next(), levels.next()) {
            (Some(first), Some(second)) => {
                let (mut highest, mut second_highest) =
                    (std::cmp::max(first, second), std::cmp::min(first, second));
                for level in levels {
                    if level > highest {
                        second_highest = highest;
                        highest = level;
                    } else if level > second_highest {
                        second_highest = level
                    }
                }
                second_highest
            }
            _ => 0,
        }
    }

    pub fn len(&self) -> usize {
        self.literals.len()
    }

    pub fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }
}
