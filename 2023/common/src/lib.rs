use miette::Diagnostic;
use nom::{error::Error as NomError, Parser};
use std::io::{Error as IoError, ErrorKind};
use thiserror::Error as ThisError;

#[derive(ThisError, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(IoError),
}
impl AocError {
    pub fn from_str(message: &str) -> Self {
        AocError::IoError(IoError::new(ErrorKind::InvalidInput, message))
    }
}

pub fn nom<'a, P, O>(mut parser: P, input: &'a str) -> miette::Result<O, AocError>
where
    P: Parser<&'a str, O, NomError<&'a str>>,
{
    match parser.parse(input.trim()) {
        Ok((remaining_input, engine)) => {
            if !remaining_input.trim().is_empty() {
                Err(AocError::from_str(
                    "Additional unparsed data at the end of input: {remaining_input}",
                ))
            } else {
                Ok(engine)
            }
        }
        Err(_) => Err(AocError::from_str("Input could not be correctly parsed.")),
    }
}
