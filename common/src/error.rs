use std::error::Error as RustError;
use std::io::Error as IoError;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Cannot convert to integer: {0}")]
    Int(ParseIntError),
    #[error("Nomming went bad: {0}")]
    Nom(String),
}

#[derive(Debug, Error)]
pub enum RunnerError {
    #[error("Not yet implemented")]
    Unimplemented,
    #[error("Unknown error: {0}")]
    Other(#[from] Box<dyn RustError>),
}

#[derive(Debug, Error)]
pub enum AocError {
    #[error("Could not fetch input: {0}")]
    Io(IoError),
    #[error("Day #{0} not a registered solution")]
    OutOfScope(u8),
    #[error("Could not parse input into data model: {0}")]
    Parse(ParseError),
    #[error("Could not run solution against data model: {0}")]
    Run(RunnerError),
}
impl From<IoError> for AocError {
    fn from(value: IoError) -> Self {
        AocError::Io(value)
    }
}
impl From<ParseError> for AocError {
    fn from(value: ParseError) -> Self {
        AocError::Parse(value)
    }
}
impl From<RunnerError> for AocError {
    fn from(value: RunnerError) -> Self {
        AocError::Run(value)
    }
}
