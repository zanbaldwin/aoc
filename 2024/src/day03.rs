use aoc_common::{ParseError, RunnerError, Solution};
use models::Instruction;
use regex::Regex;
use std::fmt::Display;

pub struct Day03 {}
impl Solution for Day03 {
    type Parsed = Vec<models::Operation>;

    fn parse(&self, input: &str) -> Result<Self::Parsed, ParseError> {
        parser::parse(input)
    }

    fn part1(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        Ok(input.iter().map(|operation| operation.product()).sum::<u64>())
    }

    fn part2(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        Ok(input.iter().filter(|o| o.enabled()).map(|o| o.product()).sum::<u64>())
    }
}

pub struct Day03WithRegex {}
impl Solution for Day03WithRegex {
    type Parsed = Vec<models::Instruction>;

    fn parse(&self, input: &str) -> Result<Self::Parsed, ParseError> {
        let re = Regex::new(r"(?:do\(\)|don't\(\)|mul\((?<first>\d{0,3}),(?<second>\d{0,3})\))").unwrap();
        Ok(re
            .captures_iter(input)
            .map(|captures| match &captures.get(0).unwrap().as_str()[..3] {
                "do(" => Instruction::Do,
                "don" => Instruction::Dont,
                "mul" => Instruction::Multiply(
                    captures.name("first").unwrap().as_str().parse().unwrap(),
                    captures.name("second").unwrap().as_str().parse().unwrap(),
                ),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>())
    }

    fn part1(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        Ok(input
            .iter()
            .filter_map(|instruction| match instruction {
                Instruction::Multiply(first, second) => Some((first * second) as u64),
                _ => None,
            })
            .sum::<u64>())
    }

    fn part2(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        let mut enabled = true;
        let mut result = 0;
        for instruction in input.into_iter() {
            match instruction {
                Instruction::Do => enabled = true,
                Instruction::Dont => enabled = false,
                Instruction::Multiply(first, second) => {
                    if enabled {
                        result += (first * second) as u64;
                    }
                },
            }
        }
        Ok(result)
    }
}

mod models {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Operation {
        input: (u32, u32),
        enabled: bool,
    }
    impl Operation {
        pub(crate) fn new(input: (u32, u32), enabled: bool) -> Self {
            Self { input, enabled }
        }

        pub(crate) fn product(&self) -> u64 {
            (self.input.0 * self.input.1) as u64
        }

        pub(crate) fn enabled(&self) -> bool {
            self.enabled
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Instruction {
        Do,
        Dont,
        Multiply(u32, u32),
    }
}

mod parser {
    use super::models::Operation;
    use aoc_common::ParseError;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{anychar, satisfy},
        combinator::map_res,
        error::Error as NomError,
        multi::{fold_many_m_n, many_till},
        sequence::{delimited, separated_pair, tuple},
        IResult,
    };

    fn parse_number(input: &str) -> IResult<&str, u32> {
        map_res(
            tuple((
                satisfy(|c| c.is_ascii_digit() && '0' != c),
                fold_many_m_n(0, 2, satisfy(|c| c.is_ascii_digit()), String::new, |mut carry, item| {
                    carry.push(item);
                    carry
                }),
            )),
            |(first, remaining)| format!("{first}{remaining}").parse(),
        )(input)
    }

    fn parse_number_pair(input: &str) -> IResult<&str, (u32, u32)> {
        separated_pair(parse_number, tag(","), parse_number)(input)
    }

    fn parse_mul(input: &str) -> IResult<&str, (u32, u32)> {
        delimited(tag("mul("), parse_number_pair, tag(")"))(input)
    }

    pub(crate) fn parse(mut input: &str) -> Result<Vec<Operation>, ParseError> {
        let mut operations = Vec::new();
        let mut enabled = true;
        // Consider this a long-winded way of achieving the `take_until_parser`
        // because the normal `take_until` only consumes a static tag.
        'take: while let Ok((remaining, (_taken, instruction))) =
            many_till(anychar::<_, NomError<_>>, alt((tag("mul("), tag("do()"), tag("don't()"))))(input)
        {
            // We want the remainder to include the instruction, so we can match again.
            let consumed = input.len() - (instruction.len() + remaining.len());
            let remaining = &input[consumed..];

            match instruction {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                "mul(" => {
                    if let Ok((remaining, operation)) = parse_mul(remaining) {
                        operations.push(Operation::new(operation, enabled));
                        input = remaining;
                        continue 'take;
                    }
                },
                _ => (),
            };
            // False positive, advance past what we just matched to
            // prevent matching the same one over and over again.
            input = &remaining[1..];
        }
        Ok(operations)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use nom::error::{Error, ErrorKind};
        use rstest::rstest;

        fn nom_error<T>(remaining: &str, kind: ErrorKind) -> Result<T, nom::Err<Error<&str>>> {
            Err(nom::Err::Error(Error::new(remaining, kind)))
        }

        #[rstest]
        #[case("1", 1, "")]
        #[case("12", 12, "")]
        #[case("123", 123, "")]
        #[case("1234", 123, "4")]
        #[case("12a", 12, "a")]
        fn test_match_number_valid(#[case] input: &str, #[case] result: u32, #[case] remaining: &str) {
            assert_eq!(Ok((remaining, result)), parse_number(input));
        }

        #[rstest]
        #[case("0", ErrorKind::Satisfy, "0")]
        #[case("", ErrorKind::Satisfy, "")]
        #[case(" 123", ErrorKind::Satisfy, " 123")]
        #[case("012", ErrorKind::Satisfy, "012")]
        fn test_match_number_invalid(#[case] input: &str, #[case] kind: ErrorKind, #[case] remaining: &str) {
            assert_eq!(nom_error(remaining, kind), parse_number(input));
        }

        #[rstest]
        #[case("12,2", (12, 2), "")]
        #[case("1,23", (1, 23), "")]
        #[case("12,23", (12, 23), "")]
        #[case("123,321", (123, 321), "")]
        #[case("123,1231", (123, 123), "1")]
        #[case("12,23a", (12, 23), "a")]
        fn test_match_pair_valid(#[case] input: &str, #[case] result: (u32, u32), #[case] remaining: &str) {
            assert_eq!(Ok((remaining, result)), parse_number_pair(input));
        }

        #[rstest]
        #[case("0", ErrorKind::Satisfy, "0")]
        #[case("", ErrorKind::Satisfy, "")]
        #[case("1, 2", ErrorKind::Satisfy, " 2")]
        #[case(" 1,2", ErrorKind::Satisfy, " 1,2")]
        #[case("1,", ErrorKind::Satisfy, "")]
        fn test_match_pair_invalid(#[case] input: &str, #[case] kind: ErrorKind, #[case] remaining: &str) {
            assert_eq!(nom_error(remaining, kind), parse_number_pair(input));
        }

        #[rstest]
        #[case("mul(12,2)", (12,2), "")]
        #[case("mul(1,23)", (1, 23), "")]
        #[case("mul(123,123)mul(123,123)", (123, 123), "mul(123,123)")]
        fn match_parse_mul_valid(#[case] input: &str, #[case] result: (u32, u32), #[case] remaining: &str) {
            assert_eq!(Ok((remaining, result)), parse_mul(input));
        }

        #[rstest]
        #[case("mul(1,023)", ErrorKind::Satisfy, "023)")]
        #[case("mmul(123,123)", ErrorKind::Tag, "mmul(123,123)")]
        #[case("1, 2", ErrorKind::Tag, "1, 2")]
        #[case("mul((1,2)", ErrorKind::Satisfy, "(1,2)")]
        #[case("mul(1,2-)", ErrorKind::Tag, "-)")]
        fn test_match_mul_invalid(#[case] input: &str, #[case] kind: ErrorKind, #[case] remaining: &str) {
            assert_eq!(nom_error(remaining, kind), parse_mul(input))
        }

        #[rstest]
        #[case("", vec![])]
        #[case("mul(1,2)", vec![(1, 2, true)])]
        #[case("mul(1,2)mul(3,4)", vec![(1, 2, true), (3, 4, true)])]
        #[case("mul(1,2) mul(3,4)", vec![(1, 2, true), (3, 4, true)])]
        #[case("xmul(1,2)mul(mul(3,4)", vec![(1, 2, true), (3, 4, true)])]
        #[case("do_not_mul(5,5)", vec![(5, 5, true)])]
        #[case("mul[3,7]", vec![])]
        fn test_parse_string(#[case] input: &str, #[case] expected: Vec<(u32, u32, bool)>) {
            let expected =
                expected.into_iter().map(|(first, second, enabled)| Operation::new((first, second), enabled)).collect();
            assert_eq!(Ok(expected), parse(input));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use models::Operation;

    const PART1_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const PART2_INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_parse() {
        let solution = Day03 {};
        let expected = vec![
            Operation::new((2, 4), true),
            Operation::new((5, 5), true),
            Operation::new((11, 8), true),
            Operation::new((8, 5), true),
        ];
        let parsed = solution.parse(PART1_INPUT).unwrap();
        assert_eq!(expected, parsed);
    }

    #[test]
    fn test_part_one() {
        let solution = Day03 {};
        let parsed = solution.parse(PART1_INPUT).unwrap();
        assert_eq!("161", solution.part1(parsed).unwrap().to_string());
    }

    #[test]
    // Make sure we aren't getting any overflows.
    fn test_part_one_overflow() {
        let solution = Day03 {};
        let parsed = solution.parse("mul(999,999)").unwrap();
        assert_eq!("998001", solution.part1(parsed).unwrap().to_string());
    }

    #[test]
    fn test_part_two() {
        let solution = Day03 {};
        let parsed = solution.parse(PART2_INPUT).unwrap();
        assert_eq!("48", solution.part2(parsed).unwrap().to_string());
    }

    #[test]
    fn test_parse_regex() {
        let solution = Day03WithRegex {};
        let expected = vec![
            Instruction::Multiply(2, 4),
            Instruction::Multiply(5, 5),
            Instruction::Multiply(11, 8),
            Instruction::Multiply(8, 5),
        ];
        let parsed = solution.parse(PART1_INPUT).unwrap();
        assert_eq!(expected, parsed);
    }

    #[test]
    fn test_part_one_regex() {
        let solution = Day03WithRegex {};
        let parsed = solution.parse(PART1_INPUT).unwrap();
        assert_eq!("161", solution.part1(parsed).unwrap().to_string());
    }

    #[test]
    // Make sure we aren't getting any overflows.
    fn test_part_one_overflow_regex() {
        let solution = Day03WithRegex {};
        let parsed = solution.parse("mul(999,999)").unwrap();
        assert_eq!("998001", solution.part1(parsed).unwrap().to_string());
    }

    #[test]
    fn test_part_two_regex() {
        let solution = Day03WithRegex {};
        let parsed = solution.parse(PART2_INPUT).unwrap();
        assert_eq!("48", solution.part2(parsed).unwrap().to_string());
    }
}
