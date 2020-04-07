pub mod clause;

use crate::{Literal, Solution, Solver, Variable};
pub use clause::Clause;

pub struct Formula {
    pub clauses: Vec<Clause>,
}

impl Formula {
    pub fn solve(self) -> Solution {
        Solver::solve_formula(self)
    }

    pub fn all_literals<'a>(&'a self) -> impl Iterator<Item = Literal> + 'a {
        self.clauses.iter().flat_map(|clause| clause.literals())
    }

    pub fn all_variables<'a>(&'a self) -> impl Iterator<Item = Variable> + 'a {
        self.all_literals().map(|literal| literal.var())
    }

    pub fn num_variables(&self) -> Variable {
        self.all_variables().max().map(|x| x + 1).unwrap_or(0)
    }

    pub fn distinct_variables(&self) -> Vec<Variable> {
        let mut vars = self.all_variables().collect::<Vec<Variable>>();
        vars.sort();
        vars.dedup();
        vars
    }
}
