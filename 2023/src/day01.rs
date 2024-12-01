use aoc_common::{ParseError, RunnerError, Solution};
use std::fmt::Display;
use std::iter;

const WORDS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub struct Day01 {}
impl Solution for Day01 {
    type Parsed = Vec<String>;

    fn parse(&self, input: &str) -> Result<Self::Parsed, ParseError> {
        Ok(input.trim().lines().map(str::to_string).collect())
    }

    fn part1(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        let total = input
            .iter()
            .filter_map(|l| {
                let nums: Vec<_> = l.chars().filter_map(|c| c.to_digit(10)).collect();
                match nums.len() {
                    0 => None,
                    _ => Some((nums.first().unwrap() * 10) + nums.last().unwrap()),
                }
            })
            .sum::<u32>();
        Ok(total)
    }

    fn part2(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        Ok(input
            .iter()
            .map(|line| {
                let mut index = 0;
                let line_iter = iter::from_fn(move || {
                    let check_line = &line[index..];
                    let mut result: Option<char> = None;
                    'found: for (i, word) in WORDS.iter().enumerate() {
                        if check_line.starts_with(word) {
                            result = Some((i + 1).to_string().chars().nth(0).unwrap());
                            break 'found;
                        }
                    }
                    index += 1;
                    result.or_else(|| check_line.chars().next())
                });
                let nums: Vec<_> = line_iter.filter_map(|c| c.to_digit(10)).collect();
                (nums.first().unwrap() * 10) + nums.last().unwrap()
            })
            .sum::<u32>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let solution = Day01 {};
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let parsed = solution.parse(input).unwrap();
        assert_eq!(vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"], parsed);
    }

    #[test]
    fn test_part_one() {
        let solution = Day01 {};
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let parsed = solution.parse(input).unwrap();
        assert_eq!("142", solution.part1(parsed).unwrap().to_string());
    }

    #[test]
    fn test_part_two() {
        let solution = Day01 {};
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let parsed = solution.parse(input).unwrap();
        assert_eq!("281", solution.part2(parsed).unwrap().to_string());
    }
}
