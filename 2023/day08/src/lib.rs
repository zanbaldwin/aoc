use std::fmt;

use parser::parse_collection;

pub mod part1;
pub mod part2;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    NotYetImplemented,
    Other(String),
    InvalidDirection(char),
    MissingPosition(String),
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

mod models {
    use crate::Error;
    use std::collections::HashMap;

    pub(crate) enum Direction {
        Left,
        Right,
    }
    impl TryFrom<char> for Direction {
        type Error = Error;
        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'L' => Ok(Self::Left),
                'R' => Ok(Self::Right),
                c => Err(Error::InvalidDirection(c)),
            }
        }
    }

    pub(crate) struct Map<'a> {
        pub(crate) left: &'a str,
        pub(crate) right: &'a str,
    }
    impl<'a> Map<'a> {
        pub fn next(&self, direction: &Direction) -> &'a str {
            match direction {
                Direction::Left => self.left,
                Direction::Right => self.right,
            }
        }
    }

    pub(crate) struct Maps<'a> {
        pub(crate) sequence: Vec<Direction>,
        pub(crate) maps: HashMap<&'a str, Map<'a>>,
    }
    impl<'a> Maps<'a> {
        pub(crate) fn process(&self) -> Result<usize, Error> {
            let mut position: &str = "AAA";
            let mut count = 0;
            'logic: loop {
                for direction in self.sequence.iter() {
                    count += 1;
                    position = self
                        .maps
                        .get(position)
                        .ok_or(Error::MissingPosition(position.to_string()))?
                        .next(direction);
                    if position == "ZZZ" {
                        break 'logic;
                    }
                }
            }
            Ok(count)
        }
    }
}

mod parser {
    use std::collections::HashMap;

    use super::models::*;
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, line_ending, one_of, space0, space1},
        combinator::{map, map_res},
        multi::{many1, separated_list1},
        sequence::{delimited, separated_pair, tuple},
        IResult,
    };

    pub(crate) fn parse_collection<'a>(input: &'a str) -> IResult<&'a str, Maps<'a>> {
        map(
            separated_pair(parse_sequence, many1(line_ending), parse_maps),
            |(sequence, maps)| Maps { sequence, maps },
        )(input)
    }

    fn parse_sequence(input: &str) -> IResult<&str, Vec<Direction>> {
        many1(map_res(one_of("LR"), |c| Direction::try_from(c)))(input)
    }

    fn parse_maps<'a>(input: &'a str) -> IResult<&'a str, HashMap<&'a str, Map<'a>>> {
        let (remaining_input, maps) = separated_list1(line_ending, parse_map)(input)?;
        let mut collection = HashMap::new();
        for (source, map) in maps {
            collection.insert(source, map);
        }
        Ok((remaining_input, collection))
    }

    fn parse_map<'a>(input: &'a str) -> IResult<&'a str, (&'a str, Map<'a>)> {
        map(
            separated_pair(
                alpha1,
                tuple((space1, tag("="), space1)),
                delimited(
                    tag("("),
                    separated_pair(alpha1, tuple((space0, tag(","), space0)), alpha1),
                    tag(")"),
                ),
            ),
            |(source, (left, right))| (source, Map { left, right }),
        )(input)
    }
}

pub(crate) fn parse<'a>(input: &'a str) -> Result<models::Maps<'a>, Error> {
    common::nom(parse_collection, input.trim())
}
