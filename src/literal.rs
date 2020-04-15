use crate::{Assignments, Evaluate, Sign, Variable};

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Literal {
    code: Variable,
}

impl Literal {
    #[inline]
    pub fn new(var: Variable, sign: impl Into<Sign>) -> Self {
        assert!(var < (Variable::max_value() >> 1));
        let sign = sign.into();
        Literal {
            code: (var << 1) | matches!(sign, Sign::Positive) as Variable,
        }
    }

    #[inline]
    pub fn var(self) -> Variable {
        self.code >> 1
    }

    #[inline]
    pub fn sign(self) -> Sign {
        ((self.code & 1) == 1).into()
    }

    pub(crate) fn code(self) -> usize {
        self.into()
    }

    pub(crate) fn from_code(code: usize) -> Self {
        code.into()
    }
}

impl Evaluate for Literal {
    fn evaluate(&self, assignments: &Assignments) -> Option<bool> {
        assignments
            .get(self.var())
            .map(|assignment| assignment.sign() == self.sign())
    }
}

impl std::ops::Not for Literal {
    type Output = Literal;

    #[inline]
    fn not(self) -> Self::Output {
        Literal {
            code: self.code ^ 1,
        }
    }
}

impl From<isize> for Literal {
    fn from(x: isize) -> Self {
        assert_ne!(x, 0, "literals can only be parsed from non-zero inputs");
        Self::new((x.abs() - 1) as Variable, x > 0)
    }
}

impl From<usize> for Literal {
    fn from(code: usize) -> Literal {
        Self { code }
    }
}

impl Into<usize> for Literal {
    fn into(self) -> usize {
        self.code
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.sign(), self.var())
    }
}

impl std::fmt::Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.sign(), self.var())
    }
}
