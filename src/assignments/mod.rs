use crate::{solver::Status, DecisionLevel, History, Literal, Sign, Variable};

pub mod assignment;
pub use assignment::Assignment;

#[derive(Clone, Debug)]
pub struct Assignments {
    assignments: Vec<Option<Assignment>>,
}

impl Assignments {
    pub fn new(num_vars: Variable) -> Self {
        Self {
            assignments: vec![None; num_vars as usize],
        }
    }

    pub fn get(&self, var: Variable) -> Option<&Assignment> {
        self.assignments[var as usize].as_ref()
    }

    pub fn assigned_at_level(&self, var: Variable, level: DecisionLevel) -> bool {
        self.get(var)
            .map(|assignment| assignment.decision_level() == level)
            .unwrap_or(false)
    }

    pub fn set(&mut self, var: Variable, assignment: Assignment, history: &mut History) -> Status {
        match self.get(var) {
            None => {
                let literal = Literal::new(var, assignment.sign());
                if assignment.decision_level() == 0 {
                    history.add_invariant(literal);
                } else {
                    history.add(literal);
                }
                self.assignments[var] = Some(assignment);
                Status::Ok
            }
            Some(existing) if existing.sign() != assignment.sign() => assignment
                .antecedent()
                .map(Status::ConflictClause)
                .unwrap_or_else(|| Status::ConflictLiteral(Literal::new(var, assignment.sign()))),
            _ => Status::Ok,
        }
    }

    pub(crate) fn set_invariant(
        &mut self,
        var: Variable,
        sign: Sign,
        history: &mut History,
    ) -> Status {
        self.set(var, Assignment::decided(sign, 0), history)
    }

    pub fn remove(&mut self, var: Variable) {
        self.assignments[var] = None;
    }

    pub fn assignments(self) -> impl Iterator<Item = (Variable, Sign)> {
        self.assignments
            .into_iter()
            .map(|assignment| assignment.unwrap().sign())
            .enumerate()
    }
}

#[cfg(test)]
impl Assignments {
    pub fn new_with(signs: Vec<Option<Sign>>) -> Self {
        let mut x = Self::new(signs.len());
        for (i, sign) in signs.into_iter().enumerate() {
            sign.map(|sign| x.set_unchecked(i, sign));
        }
        x
    }

    pub fn set_unchecked(&mut self, var: Variable, sign: Sign) {
        self.assignments[var as usize] = Some(Assignment::decided(sign, 0));
    }
}
