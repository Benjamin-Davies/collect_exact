use std::{error, fmt};

/// Error that is returned when the iterator does not have exactly the right number of items.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Error {
    /// The iterator has too few items.
    TooFewItems,
    /// The iterator has too many items.
    TooManyItems,
}

/// Error that is returned when the iterator has too few items.
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
