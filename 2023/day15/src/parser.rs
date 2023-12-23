use std::num::ParseIntError;

use crate::{
    error::Error,
    models::{Instruction, Step},
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
    sequence::tuple,
    IResult,
};

/// Parse Step
///
/// Parse a step from the initialization sequence from a string into a [`Step`].
/// Examples of steps include:
///
/// - `rn=1`
/// - `cm-`
/// - `qp=3`
/// - `cm=2`
/// - `qp-`
/// - `pc=4`
/// - `ot=9`
/// - `ab=5`
/// - `pc-`
/// - `pc=6`
/// - `ot=7`
///
/// [`Step`]: crate::models::Step
pub fn parse_step(input: &str) -> IResult<&str, Step, Error> {
    map(tuple((alpha1, parse_instruction)), |(label, instruction)| Step { label, instruction })(input)
}

/// Parse Instruction
///
/// Parse a step from the initialization sequence from a string into a
/// [`Instruction`]. Examples of instructions include:
///
/// - `=1`
/// - `-`
/// - `=2`
/// - `=3`
///
/// [`Instruction`]: crate::models::Instruction
pub fn parse_instruction(input: &str) -> IResult<&str, Instruction, Error> {
    map_res(
        alt((tuple((tag("-"), take(0usize))), tuple((tag("="), digit1)))),
        |(instruction, focal_point)| -> Result<Instruction, ParseIntError> {
            match instruction {
                "=" => Ok(Instruction::Insert(focal_point.parse()?)),
                "-" => Ok(Instruction::Remove),
                _ => unreachable!(),
            }
        },
    )(input)
}
