use crate::{aoc_error::AocError, invalid_input_error, Chunk, Engine, Line};
use nom::{
    branch::alt,
    character::complete::{char, digit1, line_ending, satisfy},
    combinator::{consumed, map},
    multi::many1,
    multi::separated_list1,
    IResult,
};

fn parse_line(input: &str) -> IResult<&str, Line> {
    let blankspace = map(consumed(many1(char('.'))), |(space_str, _spaces)| Chunk::BlankSpace(space_str));
    let partnumber = map(digit1, Chunk::PartNumber);
    let symbol = map(satisfy(|c| !c.is_ascii_digit() && c != '.' && c != '\n'), Chunk::Symbol);
    many1(alt((blankspace, partnumber, symbol)))(input)
}

/// Parse Input from AoC2023 Day 3 into Data Structure
pub(crate) fn parse(input: &str) -> miette::Result<Engine, AocError> {
    match separated_list1(line_ending, parse_line)(input.trim()) {
        Ok((remaining_input, engine)) => {
            if !remaining_input.trim().is_empty() {
                Err(invalid_input_error("Additional unparsed data at the end of input: {remaining_input}"))
            } else {
                Ok(engine)
            }
        },
        Err(_) => Err(invalid_input_error("Input could not be correctly parsed.")),
    }
}
