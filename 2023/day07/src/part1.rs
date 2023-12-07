use crate::{
    models::{Hand, HandWithoutJokers},
    parser::parse,
    Error,
};

pub fn process(input: &str) -> Result<String, Error> {
    let parsed_hands = parse(input)?;
    let mut hands: Vec<HandWithoutJokers> = parsed_hands
        .into_iter()
        .map(|hand| -> Result<HandWithoutJokers, Error> { hand.try_into() })
        .collect::<Result<Vec<_>, Error>>()?;
    hands.sort();
    let total: usize = hands
        .into_iter()
        .enumerate()
        .map(|(index, hand)| hand.score(index + 1))
        .sum();
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY07_TEST_INPUT: &str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!("6440", process(DAY07_TEST_INPUT).unwrap());
    }

    #[test]
    fn test_part1_reddit() {
        assert_eq!(
            "6592",
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
