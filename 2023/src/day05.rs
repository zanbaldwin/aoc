use aoc_common::{ParseError, RunnerError, Solution};
use std::fmt::Display;

pub struct Day05 {}
impl Solution for Day05 {
    type Parsed = models::Almanac;

    fn parse(&self, input: &str) -> Result<Self::Parsed, ParseError> {
        parser::parse(input)
    }

    fn part1(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        input
            .get_seeds_from_list()
            .iter()
            .map(|seed| input.translate_seed_to_location(*seed))
            .min()
            .ok_or_else(|| RunnerError::Custom("There should be at least one seed defined".to_string()))
    }

    fn part2(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        (1..i64::MAX)
            .find(|location| {
                input.get_seed_ranges().iter().any(|range| range.contains(&input.translate_location_to_seed(*location)))
            })
            .ok_or_else(|| RunnerError::Custom("There should be at least one matching location".to_string()))
    }
}

mod models {
    use std::{collections::HashMap, ops::Range};

    const SEED_PROCESS_ORDER: [(Category, Category); 7] = [
        (Category::Seed, Category::Soil),
        (Category::Soil, Category::Fertilizer),
        (Category::Fertilizer, Category::Water),
        (Category::Water, Category::Light),
        (Category::Light, Category::Temperature),
        (Category::Temperature, Category::Humidity),
        (Category::Humidity, Category::Location),
    ];

    #[derive(Clone, Debug, PartialEq)]
    pub struct Almanac {
        seeds: Vec<i64>,
        mappings: HashMap<(Category, Category), Mapping>,
    }
    #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
    pub(crate) enum Category {
        Seed,
        Soil,
        Fertilizer,
        Water,
        Light,
        Temperature,
        Humidity,
        Location,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub(crate) struct Mapping {
        from: Category,
        to: Category,
        transformers: Vec<Transformer>,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub(crate) struct Transformer {
        range: Range<i64>,
        shift: i64,
    }

    impl Almanac {
        pub(crate) fn new(seeds: Vec<i64>, transformer_mappings: Vec<Mapping>) -> Self {
            let mut mappings: HashMap<(Category, Category), Mapping> = HashMap::new();
            for mapping in transformer_mappings {
                mappings.insert((mapping.from, mapping.to), mapping);
            }
            Self { seeds, mappings }
        }

        fn get_mapping(&self, from: Category, to: Category) -> Option<&Mapping> {
            self.mappings.get(&(from, to))
        }

        pub(crate) fn get_seeds_from_list(&self) -> &[i64] {
            &self.seeds
        }

        pub(crate) fn get_seed_ranges(&self) -> Vec<Range<i64>> {
            assert!(self.seeds.len() % 2 == 0);
            self.seeds
                .chunks(2)
                .map(|chunk| {
                    let start = chunk[0];
                    let end = chunk[0] + chunk[1];
                    start..end
                })
                .collect()
        }

        pub(crate) fn translate_seed_to_location(&self, seed: i64) -> i64 {
            let mut result = seed;
            for (from, to) in SEED_PROCESS_ORDER {
                if let Some(mapping) = self.get_mapping(from, to) {
                    result = mapping.forwards(result);
                } else {
                    panic!("No mapping from {from:?} to {to:?}");
                }
            }
            result
        }

        pub(crate) fn translate_location_to_seed(&self, location: i64) -> i64 {
            let mut result = location;
            for (from, to) in SEED_PROCESS_ORDER.iter().rev() {
                if let Some(mapping) = self.get_mapping(*from, *to) {
                    result = mapping.backwards(result);
                } else {
                    panic!("No mapping from {from:?} to {to:?}");
                }
            }
            result
        }
    }

    impl Category {
        pub(crate) fn from_str(category: &str) -> Self {
            match category {
                "seed" => Self::Seed,
                "soil" => Self::Soil,
                "fertilizer" => Self::Fertilizer,
                "water" => Self::Water,
                "light" => Self::Light,
                "temperature" => Self::Temperature,
                "humidity" => Self::Humidity,
                "location" => Self::Location,
                _ => panic!("Unknown category: {}", category),
            }
        }
    }

    impl Mapping {
        pub(crate) fn new(heading: (Category, Category), transformers: Vec<Transformer>) -> Self {
            Self {
                from: heading.0,
                to: heading.1,
                transformers,
            }
        }

        fn forwards(&self, value: i64) -> i64 {
            for transformer in &self.transformers {
                if transformer.range.contains(&value) {
                    return value + transformer.shift;
                }
            }
            value
        }

        fn backwards(&self, value: i64) -> i64 {
            for transformer in &self.transformers {
                if transformer.range.contains(&(value - transformer.shift)) {
                    return value - transformer.shift;
                }
            }
            value
        }
    }

    impl Transformer {
        pub(crate) fn new(destination: i64, source: i64, length: i64) -> Self {
            let shift = destination - source;
            let range = source..(source + length);
            Self { range, shift }
        }
    }
}

mod parser {
    use super::models::*;
    use aoc_common::ParseError;
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, i64, line_ending, space1},
        combinator::map,
        multi::{many1, separated_list1},
        sequence::{preceded, separated_pair, terminated, tuple},
        IResult,
    };

    fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
        preceded(tuple((tag("seeds:"), space1)), separated_list1(space1, i64))(input)
    }

    fn parse_heading(input: &str) -> IResult<&str, (Category, Category)> {
        terminated(
            separated_pair(map(alpha1, Category::from_str), tag("-to-"), map(alpha1, Category::from_str)),
            tuple((space1, tag("map:"))),
        )(input)
    }

    fn parse_transformers(input: &str) -> IResult<&str, Vec<Transformer>> {
        separated_list1(
            line_ending,
            map(tuple((i64, space1, i64, space1, i64)), |(destination, _, source, _, length)| {
                Transformer::new(destination, source, length)
            }),
        )(input)
    }

    fn parse_mapping(input: &str) -> IResult<&str, Mapping> {
        map(separated_pair(parse_heading, line_ending, parse_transformers), |(heading, transformers)| {
            Mapping::new(heading, transformers)
        })(input)
    }

    fn parse_mappings(input: &str) -> IResult<&str, Vec<Mapping>> {
        let multiline_break = tuple((line_ending, many1(line_ending)));
        separated_list1(multiline_break, parse_mapping)(input)
    }

    fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
        let multiline_break = tuple((line_ending, many1(line_ending)));
        map(tuple((parse_seeds, multiline_break, parse_mappings)), |(seeds, _, mappings)| {
            Almanac::new(seeds, mappings)
        })(input)
    }

    pub(crate) fn parse(input: &str) -> Result<Almanac, ParseError> {
        parse_almanac(input).map_err(|e| ParseError::Nom(e.to_string())).map(|(_, almanac)| almanac)
    }
}

#[cfg(test)]
mod tests {
    use super::models::*;
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

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

    #[test]
    fn test_parse() {
        let solution = Day05 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!(
            Almanac::new(
                vec![79, 14, 55, 13],
                vec![
                    Mapping::new(
                        (Category::Seed, Category::Soil),
                        vec![Transformer::new(50, 98, 2), Transformer::new(52, 50, 48)]
                    ),
                    Mapping::new(
                        (Category::Soil, Category::Fertilizer),
                        vec![
                            Transformer::new(0, 15, 37),
                            Transformer::new(37, 52, 2),
                            Transformer::new(39, 0, 15),
                        ]
                    ),
                    Mapping::new(
                        (Category::Fertilizer, Category::Water),
                        vec![
                            Transformer::new(49, 53, 8),
                            Transformer::new(0, 11, 42),
                            Transformer::new(42, 0, 7),
                            Transformer::new(57, 7, 4),
                        ]
                    ),
                    Mapping::new(
                        (Category::Water, Category::Light),
                        vec![Transformer::new(88, 18, 7), Transformer::new(18, 25, 70)],
                    ),
                    Mapping::new(
                        (Category::Light, Category::Temperature),
                        vec![
                            Transformer::new(45, 77, 23),
                            Transformer::new(81, 45, 19),
                            Transformer::new(68, 64, 13),
                        ],
                    ),
                    Mapping::new(
                        (Category::Temperature, Category::Humidity),
                        vec![Transformer::new(0, 69, 1), Transformer::new(1, 0, 69)],
                    ),
                    Mapping::new(
                        (Category::Humidity, Category::Location),
                        vec![Transformer::new(60, 56, 37), Transformer::new(56, 93, 4)],
                    )
                ]
            ),
            parsed
        );
    }

    #[test]
    fn test_part_one() {
        let solution = Day05 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!("35", solution.part1(parsed).unwrap().to_string());
    }

    #[test]
    fn test_part_two() {
        let solution = Day05 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!("46", solution.part2(parsed).unwrap().to_string());
    }
}
