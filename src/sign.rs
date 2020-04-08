#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Sign {
    Positive,
    Negative,
}

impl From<bool> for Sign {
    #[inline]
    fn from(x: bool) -> Self {
        if x {
            Self::Positive
        } else {
            Self::Negative
        }
    }
}

impl std::fmt::Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", if self == &Sign::Negative { "-" } else { "" },)
    }
}

impl std::fmt::Debug for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", if self == &Sign::Positive { "+" } else { "-" },)
    }
}
