use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space0, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

use crate::{aoc_error::AocError, invalid_input_error, Scratchcard};

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

/// Parse Input from AoC2023 Day 4 into Data Structure
pub(crate) fn parse(input: &str) -> miette::Result<Vec<Scratchcard>, AocError> {
    match separated_list1(line_ending, parse_scratchcard)(input.trim()) {
        Ok((remaining_input, scratchcards)) => {
            if !remaining_input.trim().is_empty() {
                Err(invalid_input_error("Additional unparsed data at the end of input: {remaining_input}"))
            } else {
                Ok(scratchcards)
            }
        },
        Err(_) => Err(invalid_input_error("Input could not be correctly parsed.")),
    }
}
