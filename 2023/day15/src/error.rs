use nom::error::{ErrorKind, ParseError};
use std::fmt::{self, Debug};

#[derive(Debug, thiserror::Error)]
pub enum Error<I> {
    NotYetImplemented,
    Other(String),
    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for Error<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        Self::Nom(input, kind)
    }

    fn append(_input: I, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<I> From<&str> for Error<I> {
    fn from(value: &str) -> Self {
        Self::Other(value.to_string())
    }
}

impl<I> fmt::Display for Error<I>
where
    I: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other(msg) => write!(f, "Error: {}", msg),
            _ => write!(f, "{:?}", self),
        }
    }
}
