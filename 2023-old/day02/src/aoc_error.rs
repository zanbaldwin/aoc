use miette::Diagnostic;
use std::io::Error;
use thiserror::Error as ThisError;

#[derive(ThisError, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(Error),
}
