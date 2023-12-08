use std::fmt;

mod maths;
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
    use crate::{maths, Error};
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

    pub(crate) struct Pouch<'a> {
        pub(crate) sequence: Vec<Direction>,
        pub(crate) maps: HashMap<&'a str, Map<'a>>,
    }
    impl<'a> Pouch<'a> {
        fn process(
            &self,
            position: &str,
            end_checker: impl Fn(&str) -> bool,
        ) -> Result<usize, Error> {
            let mut position: &str = position;
            let mut count = 0;
            'logic: loop {
                for direction in self.sequence.iter() {
                    count += 1;
                    position = self
                        .maps
                        .get(position)
                        .ok_or(Error::MissingPosition(position.to_string()))?
                        .next(direction);
                    if end_checker(position) {
                        break 'logic;
                    }
                }
            }
            Ok(count)
        }

        pub(crate) fn human(&self) -> Result<usize, Error> {
            self.process("AAA", |position: &str| position == "ZZZ")
        }

        pub(crate) fn ghost(&self) -> Result<usize, Error> {
            let starting_positions: Vec<&str> = self
                .maps
                .keys()
                .filter(|position| position.ends_with("A"))
                .map(|position| *position)
                .collect();

            let lengths: Vec<usize> = starting_positions
                .into_iter()
                .map(|position| self.process(position, |position: &str| position.ends_with("Z")))
                .collect::<Result<Vec<_>, Error>>()?;

            // Lowest Common Multiple works because Eric Wastl is nice and has
            // coded the inputs to result in separate routes that do not
            // intersect (each starting position has only one ending position on
            // its route). If the routes intersected (for example, a figure
            // eight loop between two of the routes) then LCM simply would not
            // work.
            Ok(maths::lcm(&lengths))
        }
    }
}

mod parser {
    use std::collections::HashMap;

    use super::models::*;
    use nom::{
        bytes::complete::tag,
        character::complete::{alphanumeric1, line_ending, one_of, space0, space1},
        combinator::{map, map_res},
        multi::{many1, separated_list1},
        sequence::{delimited, separated_pair, tuple},
        IResult,
    };

    pub(crate) fn parse_pouch<'a>(input: &'a str) -> IResult<&'a str, Pouch<'a>> {
        map(
            separated_pair(parse_sequence, many1(line_ending), parse_maps),
            |(sequence, maps)| Pouch { sequence, maps },
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
        separated_pair(
            alphanumeric1,
            tuple((space1, tag("="), space1)),
            parse_routes,
        )(input)
    }

    fn parse_routes<'a>(input: &'a str) -> IResult<&'a str, Map<'a>> {
        let (remaining, (left, right)) = delimited(
            tag("("),
            separated_pair(
                alphanumeric1,
                tuple((space0, tag(","), space0)),
                alphanumeric1,
            ),
            tag(")"),
        )(input)?;
        // Doing this to remind myself that not everything has to be one massive
        // function call. Returned values can be processed in later lines, and I
        // need to remember to do that more often.
        Ok((remaining, Map { left, right }))
    }
}

pub(crate) fn parse<'a>(input: &'a str) -> Result<models::Pouch<'a>, Error> {
    common::nom(parser::parse_pouch, input)
}
