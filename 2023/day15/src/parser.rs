use crate::models::{Instruction, Step};
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{alpha1, digit1},
    combinator::map,
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
pub fn parse_step(input: &str) -> IResult<&str, Step> {
    map(
        tuple((alpha1, parse_instruction)),
        |(label, instruction)| Step { label, instruction },
    )(input)
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
pub fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (remaining, (instruction, focal_length)) =
        alt((tuple((tag("-"), take(0usize))), tuple((tag("="), digit1))))(input)?;

    let instruction = match instruction {
        "=" => {
            let focal_length = match focal_length.parse() {
                Ok(fl) => fl,
                Err(_) => {
                    // Can't figure out how to get custom error types working in Nom :(
                    // return Err(make_error(input, ErrorKind::Digit)),
                    panic!("Could not parse focal length.");
                }
            };
            Instruction::Insert(focal_length)
        }
        "-" => Instruction::Remove,
        _ => {
            // return make_error(input, ErrorKind::NoneOf),
            panic!("Invalid instruction.");
        }
    };

    Ok((remaining, instruction))
}
