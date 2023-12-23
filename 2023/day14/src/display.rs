use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::complete,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

use crate::{
    error::Error,
    models::{Platform, Position, Rock},
};
use std::fmt::Display;

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Cube => '#',
                Self::Sphere => 'O',
            }
        )
    }
}

impl TryFrom<&str> for Position {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        fn parse(input: &str) -> IResult<&str, (&str, &str)> {
            complete(delimited(tag("("), separated_pair(digit1, tuple((space0, tag(","), space0)), digit1), tag(")")))(
                input.trim(),
            )
        }

        let (_, (x, y)) = parse(value).map_err(|_| Error::CouldNotParsePosition)?;
        Ok(Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::with_capacity(self.height * (self.width + 1));
        for y in 1..=self.height {
            for x in 1..=self.width {
                let position = Position { x, y };
                if let Some(rock) = self.rocks.get(&position) {
                    result.push(match rock {
                        Rock::Sphere => 'O',
                        Rock::Cube => '#',
                    });
                } else {
                    result.push('.');
                }
            }
            result.push('\n');
        }
        write!(f, "{result}")
    }
}
