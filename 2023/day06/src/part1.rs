use common::AocError;

pub fn process(_input: &str) -> miette::Result<String, AocError> {
    Err("Not yet implemented.".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY06_TEST_INPUT: &str = "Time:      7  15   30\nDistance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!("288", process(DAY06_TEST_INPUT).unwrap());
    }
}
