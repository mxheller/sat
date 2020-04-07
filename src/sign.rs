#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
