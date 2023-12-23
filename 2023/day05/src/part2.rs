use crate::parse;
use common::AocError;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let almanac = parse(input)?;
    let seed_ranges = almanac.get_seed_ranges();
    let minimum_location = (1..i64::MAX)
        .find(|location| {
            let potential_seed = almanac.translate_location_to_seed(*location);
            seed_ranges.iter().any(|range| range.contains(&potential_seed))
        })
        .expect("there to be at least one matching location");
    Ok(minimum_location.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const DAY05_TEST_INPUT: &str = include_str!("../test.txt");

    #[rstest]
    #[case(79, 82)]
    #[case(14, 43)]
    #[case(55, 86)]
    #[case(13, 35)]
    fn test_translation(#[case] seed: i64, #[case] location: i64) {
        let almanac = parse(DAY05_TEST_INPUT).unwrap();
        assert_eq!(location, almanac.translate_seed_to_location(seed));
        assert_eq!(seed, almanac.translate_location_to_seed(location));
    }

    #[test]
    fn test_part2() {
        assert_eq!("46", process(DAY05_TEST_INPUT).unwrap());
    }
}
