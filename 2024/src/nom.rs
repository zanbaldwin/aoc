use ::nom::{
    error::{ErrorKind as NomErrorKind, ParseError as NomParseError},
    multi::many_till,
    Err as NomErr, IResult, InputIter, InputLength, InputTake, Parser as NomParser, Slice,
};
use std::ops::{RangeFrom, RangeTo};

/// Only consumes input if the child parser successfully matches (note: the
/// input must implement the Copy trait).
///
/// # Arguments
/// * `parser` The child parser to apply.
///
/// ```rust
/// use nom::{Err, error::ErrorKind};
/// use nom::sequence::terminated;
/// use nom::bytes::complete::tag;
/// use aoc_2024::all_or_nothing
///
/// let mut child = terminated(tag("abc"), tag("efg"));
/// let mut parser = all_or_nothing(terminated(tag("abc"), tag("efg")));
///
/// assert_eq!(child("abcefj"), Err(Err::Error(("efj", ErrorKind::Tag))));
/// assert_eq!(parser("abcefj"), Err(Err::Error(("abcefj", ErrorKind::Satisfy))));
/// ```
pub(crate) fn all_or_nothing<I: Copy, O, E, F>(mut parser: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    E: NomParseError<I>,
    F: NomParser<I, O, E>,
{
    move |input: I| {
        let original = input;
        match parser.parse(input) {
            Ok((remaining, res)) => Ok((remaining, res)),
            Err(NomErr::Incomplete(needed)) => Err(NomErr::Incomplete(needed)),
            // It would be really nice to re-use the error kind from the parse
            // error, but we referenced a trait which doesn't expose the ErrorKind :(
            Err(_) => Err(NomErr::Error(E::from_error_kind(original, NomErrorKind::Satisfy))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_or_nothing() {
        use nom::bytes::complete::tag;
        use nom::sequence::terminated;
        use nom::{error::ErrorKind, Err};

        let mut child = terminated(tag("abc"), tag("efg"));
        assert_eq!(child("abcefghij"), Ok(("hij", "abc")));
        assert_eq!(child("abcefj"), Err(Err::Error(("efj", ErrorKind::Tag))));

        let child = terminated(tag("abc"), tag("efg"));
        let mut parser = all_or_nothing(child);
        assert_eq!(parser("abcefghij"), Ok(("hij", "abc")));
        assert_eq!(parser("abcefj"), Err(Err::Error(("abcefj", ErrorKind::Satisfy))));
    }
}
