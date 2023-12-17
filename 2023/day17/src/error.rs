use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    NotYetImplemented,
    Other(String),
    CouldNotDetermineCityWidth,
    CouldNotParseBlockCost,
    InvalidCityBlock,
    ExhaustiveSearch,
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

// If today's Nom parsers have scenarios where they need to throw custom errors,
// then implement these on the Error enum:
//
// use nom::error::{ErrorKind, FromExternalError, ParseError};
// impl ParseError<&str> for Error {
//     fn from_error_kind(_input: &str, kind: ErrorKind) -> Self {
//         Self::Nom(kind)
//     }
//
//     fn append(_input: &str, _kind: ErrorKind, other: Self) -> Self {
//         other
//     }
// }
// // Required to implement trait `FromExternalError` to use custom errors with
// // functions like `nom::combinator::map_res()`.
// impl<E> FromExternalError<&str, E> for Error {
//     fn from_external_error(_input: &str, kind: ErrorKind, _: E) -> Self {
//         Self::Nom(kind)
//     }
// }
