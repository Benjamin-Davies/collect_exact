use std::{error, fmt};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Error {
    TooFewItems,
    TooManyItems,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::TooFewItems => write!(f, "too few items"),
            Error::TooManyItems => write!(f, "too many items"),
        }
    }
}
