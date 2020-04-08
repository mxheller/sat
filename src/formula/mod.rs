pub mod clause;

use crate::{solver::Solution, Literal, Sign, Solver, Variable};
pub use clause::Clause;

pub struct Formula {
    pub clauses: Vec<Clause>,
}

impl Formula {
    pub fn parse(lines: impl IntoIterator<Item = impl AsRef<str>>) -> Result<Self, String> {
        let clauses = lines
            .into_iter()
            .filter(|l| !l.as_ref().starts_with('c') && !l.as_ref().starts_with('p'))
            .map(|l| l.as_ref().parse::<Clause>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("{}", e))?;
        Ok(Self { clauses })
    }

    pub fn parse_and_solve(
        lines: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<Solution<impl IntoIterator<Item = (Variable, Sign)>>, String> {
        Self::parse(lines).map(Solver::solve_formula)
    }

    pub fn solve(self) -> Solution<impl IntoIterator<Item = (Variable, Sign)>> {
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
