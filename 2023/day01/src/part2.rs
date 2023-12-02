use crate::aoc_error::AocError;

pub fn process(_input: &str) -> miette::Result<String, AocError> {
    Err(AocError::IoError(::std::io::Error::new(
        ::std::io::ErrorKind::Other, 
        "Not yet implemented.",
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input: &'static str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!("281", process(input).unwrap());
    }

    #[test]
    fn test_word_order() {
        let line = "xtwone3four";
        assert_eq!("24", process(line).unwrap());
    }
}