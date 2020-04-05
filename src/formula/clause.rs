use crate::{formula::Literal, Variable};

pub struct Clause {}

impl Clause {
    pub fn literals(&self) -> impl Iterator<Item = &Literal> {
        unimplemented!();
        [].iter()
    }

    pub fn variables<'a>(&'a self) -> impl Iterator<Item = Variable> + 'a {
        self.literals().map(Literal::var)
    }
}
