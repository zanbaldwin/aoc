use common::AocError;

pub fn process(_input: &str) -> miette::Result<String, AocError> {
    Err("Not yet implemented.".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY07_TEST_INPUT: &str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!("6440", process(DAY07_TEST_INPUT).unwrap());
    }
}
