use aoc_common::{ParseError, RunnerError, Solution};
use std::{collections::HashMap, fmt::Display};

pub struct Day04 {}
impl Solution for Day04 {
    type Parsed = Vec<models::Scratchcard>;

    fn parse(&self, input: &str) -> Result<Self::Parsed, ParseError> {
        parser::parse(input)
    }

    fn part1(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        Ok(input
            .iter()
            .map(|scratchcard| scratchcard.num_matches())
            .filter(|n| *n > 0)
            .map(|n| 2u32.pow(n - 1))
            .sum::<u32>())
    }

    fn part2(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        let mut tally: HashMap<u32, u32> = HashMap::new();
        for scratchcard in input {
            let count_of_current_card = tally.entry(scratchcard.id()).or_insert(1).clone();
            let num_matches_of_current_card = scratchcard.num_matches();
            for i in 1..=num_matches_of_current_card {
                let copy_number = scratchcard.id() + i;
                let copy_count = tally.entry(copy_number).or_insert(1);
                *copy_count = *copy_count + count_of_current_card;
            }
        }
        Ok(tally.iter().map(|(_, count)| *count).sum::<u32>())
    }
}

mod models {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Scratchcard {
        id: u32,
        winning_numbers: Vec<u32>,
        playing_numbers: Vec<u32>,
    }
    impl Scratchcard {
        pub(crate) fn new(id: u32, winning_numbers: Vec<u32>, playing_numbers: Vec<u32>) -> Self {
            Self { id, winning_numbers, playing_numbers }
        }

        pub(crate) fn id(&self) -> u32 {
            self.id
        }

        pub(crate) fn num_matches(&self) -> u32 {
            self.playing_numbers.iter().filter(|&number| self.winning_numbers.contains(number)).count() as u32
        }
    }
}

mod parser {
    use super::models::*;
    use aoc_common::ParseError;
    use nom::{
        bytes::complete::tag,
        character::complete::{digit1, line_ending, space0, space1},
        combinator::map,
        multi::separated_list1,
        sequence::{separated_pair, tuple},
        IResult,
    };

    fn parse_list_of_numbers(input: &str) -> IResult<&str, Vec<u32>> {
        map(separated_list1(space1, digit1), |numbers| {
            numbers
                .iter()
                // We already know that the &str only contain digits because of the
                // parser before this. Unwrap.
                .map(|number: &&str| number.parse().unwrap())
                .collect()
        })(input)
    }

    fn parse_scratchcard(input: &str) -> IResult<&str, Scratchcard> {
        map(
            tuple((
                tag("Card"),
                space1,
                digit1,
                tag(":"),
                space1,
                separated_pair(parse_list_of_numbers, tuple((space0, tag("|"), space0)), parse_list_of_numbers),
            )),
            |(_, _, card_number, _, _, (winning_numbers, playing_numbers))| {
                Scratchcard::new(
                    // Already know that card_numbers only contain digits because of
                    // previous parser. Just unwrap.
                    card_number.parse().unwrap(),
                    winning_numbers,
                    playing_numbers,
                )
            },
        )(input)
    }

    pub(crate) fn parse(input: &str) -> Result<Vec<Scratchcard>, ParseError> {
        separated_list1(line_ending, parse_scratchcard)(input.trim())
            .map_err(|e| ParseError::Nom(e.to_string()))
            .map(|(_, scratchcards)| scratchcards)
    }
}

#[cfg(test)]
mod tests {
    use models::Scratchcard;

    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_parse() {
        let solution = Day04 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!(
            vec![
                Scratchcard::new(1, vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]),
                Scratchcard::new(2, vec![13, 32, 20, 16, 61], vec![61, 30, 68, 82, 17, 32, 24, 19]),
                Scratchcard::new(3, vec![1, 21, 53, 59, 44], vec![69, 82, 63, 72, 16, 21, 14, 1]),
                Scratchcard::new(4, vec![41, 92, 73, 84, 69], vec![59, 84, 76, 51, 58, 5, 54, 83]),
                Scratchcard::new(5, vec![87, 83, 26, 28, 32], vec![88, 30, 70, 12, 93, 22, 82, 36]),
                Scratchcard::new(6, vec![31, 18, 13, 56, 72], vec![74, 77, 10, 23, 35, 67, 36, 11]),
            ],
            parsed
        );
    }

    #[test]
    fn test_part_one() {
        let solution = Day04 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!("13", solution.part1(parsed).unwrap().to_string());
    }

    #[test]
    fn test_part_two() {
        let solution = Day04 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!("30", solution.part2(parsed).unwrap().to_string());
    }
}
