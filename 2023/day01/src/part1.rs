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
    fn test_part1() {
        let input: &'static str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!("142", process(input).unwrap());
    }
}