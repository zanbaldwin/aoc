use common::AocError;

pub fn process(_input: &str) -> miette::Result<String, AocError> {
    Err("Not yet implemented.".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY07_TEST_INPUT: &str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";

    #[test]
    fn test_part2() {
        assert_eq!("5905", process(DAY07_TEST_INPUT).unwrap());
    }

    #[test]
    fn test_part2_reddit() {
        assert_eq!(
            "6839",
            process(
                "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41"
            )
            .unwrap()
        );
    }
}
