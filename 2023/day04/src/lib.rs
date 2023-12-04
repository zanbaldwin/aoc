use aoc_error::AocError;
use std::io::{Error, ErrorKind};

pub mod aoc_error;
mod parser;
pub mod part1;
pub mod part2;

fn invalid_input_error(message: &str) -> AocError {
    AocError::IoError(Error::new(ErrorKind::InvalidInput, message))
}

#[derive(Debug)]
pub(crate) struct Scratchcard {
    id: u32,
    winning_numbers: Vec<u32>,
    playing_numbers: Vec<u32>,
}
impl Scratchcard {
    pub fn new(id: u32, winning_numbers: Vec<u32>, playing_numbers: Vec<u32>) -> Self {
        Self {
            id,
            winning_numbers,
            playing_numbers,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn num_matches(&self) -> u32 {
        self.playing_numbers
            .iter()
            .filter(|&number| self.winning_numbers.contains(number))
            .count() as u32
    }
}
