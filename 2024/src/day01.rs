use aoc_common::{ParseError, RunnerError, Solution};
use std::fmt::Display;

pub struct Day01 {}
impl Solution for Day01 {
    type Parsed = (Vec<isize>, Vec<isize>);

    fn parse(&self, input: &str) -> Result<Self::Parsed, ParseError> {
        let mut first = Vec::new();
        let mut second = Vec::new();
        input.trim().lines().for_each(|line| {
            let mut words = line.split_whitespace();
            if let (Some(a), Some(b)) = (words.next(), words.next()) {
                if let (Ok(a), Ok(b)) = (a.parse(), b.parse()) {
                    first.push(a);
                    second.push(b);
                }
            }
        });
        Ok((first, second))
    }

    fn part1(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        let (mut first, mut second) = input;
        first.sort();
        second.sort();
        Ok(first.into_iter().zip(second).map(|(a, b)| (a - b).abs()).sum::<isize>())
    }

    fn part2(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        let (first, second) = input;
        Ok(first.iter().map(|a| a * (second.iter().filter(|b| b == &a).count() as isize)).sum::<isize>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_parse() {
        let solution = Day01 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!((vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]), parsed);
    }

    #[test]
    fn test_part1_example() {
        let solution = Day01 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!("11".to_string(), solution.part1(parsed).unwrap().to_string());
    }

    #[test]
    fn test_part2_example() {
        let solution = Day01 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!("31".to_string(), solution.part2(parsed).unwrap().to_string());
    }
}
