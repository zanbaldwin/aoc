use miette::Diagnostic;
use nom::{
    combinator::complete,
    error::{Error as NomError, ParseError},
    Err as NomErr, Parser,
};
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

/// Apply Nom parser to input string and return a result.
///
/// This function is to be used when using Nom's built-in error: when your
/// parsers return `IResult<I, O>`.
pub fn nom<'input, P, O, E>(parser: P, input: &'input str) -> Result<O, E>
where
    P: Parser<&'input str, O, NomError<&'input str>>,
    E: From<&'static str>,
{
    return match complete(parser).parse(input.trim()) {
        Ok((_, result)) => Ok(result),
        Err(_status) => {
            // We need a way on constructing an error from within this function
            // without stating the exact type. Best I can think up with for now
            // is just to construct it from a string (meaning that it needs to
            // implement `From<&str>`).
            Err("Input could not be correctly parsed.".into())
        },
    };
}

/// Apply Nom parser to input string and return a result using a custom error
///
/// This function is to be used when using a custom Error enum/struct: when your
/// parsers return `IResult<I, O, E>`.
pub fn nom_custom<'input, P, O, E>(parser: P, input: &'input str) -> Result<O, E>
where
    P: Parser<&'input str, O, E>,
    E: ParseError<&'input str>,
{
    match complete(parser).parse(input.trim()) {
        Ok((_, result)) => Ok(result),
        Err(status) => match status {
            NomErr::Failure(e) | NomErr::Error(e) => Err(e),
            // The parser was wrapped in a `complete` parser, so there should
            // never be an Incomplete variant.
            NomErr::Incomplete(_) => unreachable!(),
        },
    }
}
