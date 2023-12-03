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
        let input: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process(input).unwrap());
    }
}
