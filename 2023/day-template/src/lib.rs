use aoc_error::AocError;
use std::io::{Error, ErrorKind};

pub mod aoc_error;
pub mod part1;
pub mod part2;

fn invalid_input_error(message: &str) -> AocError {
    AocError::IoError(Error::new(ErrorKind::InvalidInput, message))
}
