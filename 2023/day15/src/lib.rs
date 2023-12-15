//! Advent of Code 2023: Day 15
//!
//! An implementation of Day 15's challenge from [Advent of Code] 2023 in Rust
//! by [Zan Baldwin].
//!
//! ```
//! use day15::part1::process;
//!
//! // Replace this with your own input.
//! // @see https://adventofcode.com/2023/day/15
//! let my_input = include_str!("../input.txt");
//! let answer: String = process(my_input).unwrap();
//!
//! println!("Answer to Day 15 (Part 1) is: {answer}");
//! ```
//!
//! [Advent of Code]: https://adventofcode.com
//! [Zan Baldwin]: https://zanbaldwin.com

use error::Error;
use models::Step;
use nom::combinator::complete;

pub mod error;
pub mod models;
pub mod parser;
pub mod part1;
pub mod part2;

/// Reindeer HASH
///
/// An implementation of the Holiday ASCII String Helper algorithm, found in
/// appendix 1A.
pub fn reindeer_hash(step: &str) -> u32 {
    let mut hash: u32 = 0;
    for c in step.chars() {
        if c.is_whitespace() {
            continue;
        }
        hash += c as u32;
        hash *= 17;
        hash %= 256;
    }
    hash
}

/// Hash Sum
///
/// Runs the Reindeer HASH on all steps in the initialization sequence and
/// returns the sum of all hashes.
pub fn hash_sum(input: &str) -> u32 {
    input.split(',').map(reindeer_hash).sum()
}

/// Parse initialization sequence step
///
/// Parses a step from the initialization sequence from a string into a [`Step`]
/// model.
///
/// [`Step`]: crate::models::Step
pub fn parse(input: &str) -> Result<Step, Error<&str>> {
    common::nom(complete(parser::parse_step), input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn test_elf_hash(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(expected, reindeer_hash(input));
        assert_eq!(expected, hash_sum(input));
    }
}
