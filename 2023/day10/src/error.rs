use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    NotYetImplemented,
    Other(String),
    InvalidPipeCharacter(char),
    NoStartingPosition,
    NoTraversalFound,
    InvalidCircuit,
    CellParseError,
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
