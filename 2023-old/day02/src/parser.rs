use crate::{
    aoc_error::AocError,
    models::{Draw, Game, Round},
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::io::{Error, ErrorKind};

fn parse_draw(input: &str) -> IResult<&str, Draw> {
    map(
        tuple((
            map_res(digit1, |s: &str| s.parse::<u16>()),
            multispace1,
            //alpha1,
            alt((tag("red"), tag("green"), tag("blue"))),
        )),
        |(amount, _, colour): (u16, &str, &str)| Draw::from_str(amount, colour),
    )(input)
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    map(separated_list1(tag(", "), parse_draw), |draws| {
        Round::new(draws)
    })(input)
}

fn parse_rounds(input: &str) -> IResult<&str, Vec<Round>> {
    separated_list1(tag("; "), parse_round)(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    map(
        tuple((tag("Game "), digit1, tag(": "), parse_rounds)),
        |(_, id, _, rounds)| Game::new(id.parse().unwrap(), rounds),
    )(input)
}

pub(crate) fn parse(input: &str) -> Result<Vec<Game>, AocError> {
    match separated_list1(line_ending, parse_game)(input) {
        Ok((remaining_input, games)) => {
            if !remaining_input.trim().is_empty() {
                Err(AocError::IoError(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Additional unparsed data at the end of input: {remaining_input}"),
                )))
            } else {
                Ok(games)
            }
        }
        Err(_) => Err(AocError::IoError(Error::new(
            ErrorKind::InvalidInput,
            "Input could not be correctly parsed.",
        ))),
    }
}
