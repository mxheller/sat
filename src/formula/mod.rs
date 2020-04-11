pub mod clause;

use crate::{solver::Solution, Literal, Sign, Solver, Variable};
pub use clause::Clause;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

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
        Self::parse(lines).and_then(Solver::solve_formula)
    }

    pub fn parse_and_solve_file(
        path: impl AsRef<Path>,
    ) -> Result<Solution<impl IntoIterator<Item = (Variable, Sign)>>, String> {
        let lines = File::open(path)
            .map(|f| BufReader::new(f).lines().filter_map(Result::ok))
            .map_err(|e| format!("{}", e))?;

        Formula::parse_and_solve(lines)
    }

    pub fn solve(self) -> Result<Solution<impl IntoIterator<Item = (Variable, Sign)>>, String> {
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

impl From<Vec<Vec<Literal>>> for Formula {
    fn from(clauses: Vec<Vec<Literal>>) -> Self {
        Self {
            clauses: clauses.into_iter().map(|v| v.into()).collect(),
        }
    }
}

impl From<Vec<Vec<isize>>> for Formula {
    fn from(clauses: Vec<Vec<isize>>) -> Self {
        clauses
            .into_iter()
            .map(|clause| clause.into_iter().map(Literal::from).collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .into()
    }
}
