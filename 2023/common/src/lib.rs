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
impl From<&str> for AocError {
    fn from(value: &str) -> Self {
        AocError::IoError(IoError::new(ErrorKind::InvalidInput, value))
    }
}

pub fn nom<'a, P, O, E>(mut parser: P, input: &'a str) -> miette::Result<O, E>
where
    P: Parser<&'a str, O, NomError<&'a str>>,
    E: From<&'static str>,
{
    match parser.parse(input.trim()) {
        Ok((remaining_input, engine)) => {
            if !remaining_input.trim().is_empty() {
                Err("Additional unparsed data at the end of input: {remaining_input}".into())
            } else {
                Ok(engine)
            }
        }
        Err(_) => Err("Input could not be correctly parsed.".into()),
    }
}
