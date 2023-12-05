use crate::parse;
use common::AocError;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let almanac = parse(input)?;
    let minimum = almanac
        .get_seeds_from_ranges()
        .iter()
        .map(|seed| almanac.translate_seed_to_location(*seed))
        .min()
        .expect("There to be at least one seed");
    Ok(minimum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!("46", process(input).unwrap());
    }
}
