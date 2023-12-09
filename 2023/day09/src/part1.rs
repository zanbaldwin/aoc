use crate::error::Error;

pub fn process(_input: &str) -> miette::Result<String, Error> {
    Err(Error::NotYetImplemented)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY09_TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        assert_eq!("114", process(DAY09_TEST_INPUT).unwrap());
    }
}
