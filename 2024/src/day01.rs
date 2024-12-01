use aoc_common::{ParseError, RunnerError, Solution};
use std::fmt::Display;

pub struct Day01 {}
impl Solution for Day01 {
    type Parsed = i32;

    fn day(&self) -> u8 {
        1
    }

    fn parse(&self, input: &str) -> Result<Self::Parsed, ParseError> {
        let result = input.trim().parse().map_err(ParseError::Int)?;
        Ok(result)
    }

    fn part1(&self, input: &Self::Parsed) -> Result<impl Display, RunnerError> {
        Ok(input)
    }

    fn part2(&self, input: &Self::Parsed) -> Result<impl Display, RunnerError> {
        Ok(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive() {
        let solution = Day01 {};
        assert_eq!(42, solution.parse("42").unwrap());
    }
}
