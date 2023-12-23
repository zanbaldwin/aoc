use std::collections::HashMap;

use crate::{aoc_error::AocError, parser::parse};

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let scratchcards = parse(input)?;
    let mut tally: HashMap<u32, u32> = HashMap::new();

    for scratchcard in scratchcards {
        let count_of_current_card = *tally.entry(scratchcard.id()).or_insert(1);
        let num_matches_of_current_card = scratchcard.num_matches();
        for i in 1..=num_matches_of_current_card {
            let copy_number = scratchcard.id() + i;
            let copy_count = tally.entry(copy_number).or_insert(1);
            *copy_count += count_of_current_card;
        }
    }

    let total_cards: u32 = tally.values().copied().sum();
    Ok(total_cards.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", process(input).unwrap());
    }
}
