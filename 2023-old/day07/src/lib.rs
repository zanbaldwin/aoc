use core::fmt;

pub(crate) mod models;
pub mod part1;
pub mod part2;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    NotYetImplemented,
    Other(String),
    InvalidCardCharacter(char),
    WrongNumberOfCards(usize),
}
impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Other(value.to_string())
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other(msg) => write!(f, "Error: {}", msg),
            _ => write!(f, "{:?}", self),
        }
    }
}

mod parser {
    use crate::Error;

    use super::models::*;
    use nom::{
        bytes::complete::take,
        character::complete::{digit1, line_ending, space1},
        combinator::{map, verify},
        multi::separated_list1,
        sequence::separated_pair,
        IResult,
    };

    const VALID_CARD_CHARACTERS: &str = "23456789TJQKA";

    pub(crate) fn parse(input: &str) -> Result<Vec<ParsedHand>, Error> {
        common::nom(parse_playlist, input)
    }

    fn parse_playlist(input: &str) -> IResult<&str, Vec<ParsedHand>> {
        separated_list1(line_ending, parse_hand)(input)
    }

    fn parse_hand(input: &str) -> IResult<&str, ParsedHand> {
        map(
            separated_pair(
                verify(take(5usize), |cards: &str| {
                    cards.chars().all(|c| VALID_CARD_CHARACTERS.contains(c))
                }),
                space1,
                digit1,
            ),
            |(cards, bid)| ParsedHand::new(cards, bid.parse().unwrap()).unwrap(),
        )(input)
    }
}
