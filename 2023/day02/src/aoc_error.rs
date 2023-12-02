use miette::Diagnostic;
use thiserror::Error as ThisError;
use std::io::Error;

#[derive(ThisError, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(Error),
}