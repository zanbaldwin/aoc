use aoc_common::{ParseError, RunnerError, Solution};
use std::fmt::Display;

pub struct Day {}
impl Solution for Day {
    type Parsed = &str;

    fn parse(&self, input: &str) -> Result<Self::Parsed, ParseError> {
        todo!()
    }

    fn part1(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        todo!()
    }

    fn part2(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn test_parse() {
        let solution = Day {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!("", parsed);
    }

    #[test]
    fn test_part_one() {
        let solution = Day {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!("", solution.part1(parsed).unwrap().to_string());
    }

    #[test]
    fn test_part_two() {
        let solution = Day {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!("", solution.part2(parsed).unwrap().to_string());
    }
}
