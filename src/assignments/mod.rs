use crate::{
    formula::Clause, solver::Status, trimmed_formula::TrimmedFormula, History, Literal, Sign,
    Variable,
};

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

    #[cfg(test)]
    pub fn new_with(signs: Vec<Option<Sign>>) -> Self {
        let mut x = Self::new(signs.len());
        for (i, sign) in signs.into_iter().enumerate() {
            sign.map(|sign| x.set_unchecked(i, sign));
        }
        x
    }

    pub fn get(&self, var: Variable) -> Option<&Assignment> {
        self.assignments[var as usize].as_ref()
    }

    #[cfg(test)]
    pub fn set_unchecked(&mut self, var: Variable, sign: Sign) {
        self.assignments[var as usize] = Some(Assignment::decided(sign, 0));
    }

    pub fn set(
        &mut self,
        var: Variable,
        assignment: Assignment,
        formula: &TrimmedFormula,
        history: &mut History,
    ) -> Status {
        match self.get(var) {
            None => {
                if assignment.decision_level() == 0 {
                    history.add_invariant();
                } else {
                    history.add(Literal::new(var, assignment.sign()));
                }
                self.assignments[var] = Some(assignment);
                Status::Ok
            }
            Some(existing) if existing.sign() != assignment.sign() => {
                println!("Variable {} already assigned!", var);
                let conflict = assignment
                    .antecedent()
                    .map(|idx| formula[idx].get_literals())
                    .unwrap_or_else(|| Clause::from(Literal::new(var, assignment.sign())));
                Status::Conflict(conflict)
            }
            _ => Status::Ok,
        }
    }

    pub(crate) fn set_invariant(
        &mut self,
        var: Variable,
        sign: Sign,
        formula: &TrimmedFormula,
        history: &mut History,
    ) -> Status {
        self.set(var, Assignment::decided(sign, 0), formula, history)
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
