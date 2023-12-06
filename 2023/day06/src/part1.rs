use common::AocError;

use crate::parser::parse_races;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let races = parse_races(input)?;

    let total: usize = races
        .into_iter()
        .filter_map(|race| race.get_beating_range())
        // `.len()` is not available on Range<u64> because the number could be
        // larger than what 32bit systems can hold. But `.into_iter().count()`
        // should get the job done just as well.
        .map(|range| range.into_iter().count())
        .product();

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY06_TEST_INPUT: &str = "Time:      7  15   30\nDistance:  9  40  200";

    #[test]
    fn test_part1() {
        let result = process(DAY06_TEST_INPUT).unwrap();
        assert_eq!("288", result);
    }
}
