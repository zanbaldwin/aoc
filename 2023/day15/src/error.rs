use nom::error::{ErrorKind, FromExternalError, ParseError};
use std::fmt::{self, Debug};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    NotYetImplemented,
    Other(String),
    Nom(ErrorKind),
}
impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Other(value.to_string())
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other(msg) => write!(f, "Error: {}", msg),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl ParseError<&str> for Error {
    fn from_error_kind(_input: &str, kind: ErrorKind) -> Self {
        Self::Nom(kind)
    }

    fn append(_input: &str, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}
// Needed to use custom errors with functions like `nom::combinator::map_res()`.
impl<E> FromExternalError<&str, E> for Error {
    fn from_external_error(_input: &str, kind: ErrorKind, _: E) -> Self {
        Self::Nom(kind)
    }
}
