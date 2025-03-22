use std::{error, fmt};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Error {
    TooFewItems,
    TooManyItems,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PrefixError;

impl error::Error for Error {}
impl error::Error for PrefixError {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::TooFewItems => write!(f, "too few items"),
            Error::TooManyItems => write!(f, "too many items"),
        }
    }
}

impl fmt::Display for PrefixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "too few items")
    }
}

impl From<PrefixError> for Error {
    fn from(PrefixError: PrefixError) -> Self {
        Error::TooFewItems
    }
}
