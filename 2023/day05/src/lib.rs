pub mod part1;
pub mod part2;

use common::AocError;

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

    #[derive(Debug)]
    pub(crate) struct Almanac {
        seeds: Vec<i64>,
        mappings: HashMap<(Category, Category), Mapping>,
    }
    impl Almanac {
        pub fn new(seeds: Vec<i64>, transformer_mappings: Vec<Mapping>) -> Self {
            let mut mappings: HashMap<(Category, Category), Mapping> = HashMap::new();
            for mapping in transformer_mappings {
                mappings.insert((mapping.from, mapping.to), mapping);
            }
            Self { seeds, mappings }
        }

        pub fn get_mapping(&self, from: Category, to: Category) -> Option<&Mapping> {
            self.mappings.get(&(from, to))
        }

        pub fn get_seeds_from_list(&self) -> &[i64] {
            &self.seeds
        }

        pub fn get_seed_ranges(&self) -> Vec<Range<i64>> {
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

        pub fn translate_seed_to_location(&self, seed: i64) -> i64 {
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

        pub fn translate_location_to_seed(&self, location: i64) -> i64 {
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
    impl Category {
        pub fn from_str(category: &str) -> Self {
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

    #[derive(Debug)]
    pub(crate) struct Mapping {
        from: Category,
        to: Category,
        transformers: Vec<Transformer>,
    }
    impl Mapping {
        pub fn new(heading: (Category, Category), transformers: Vec<Transformer>) -> Self {
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

    #[derive(Debug)]
    pub(crate) struct Transformer {
        range: Range<i64>,
        shift: i64,
    }
    impl Transformer {
        pub fn new(destination: i64, source: i64, length: i64) -> Self {
            let shift = destination - source;
            let range = source..(source + length);
            Self { range, shift }
        }
    }
}

mod parser {
    use super::models::*;
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

    pub(crate) fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
        let multiline_break = tuple((line_ending, many1(line_ending)));
        map(tuple((parse_seeds, multiline_break, parse_mappings)), |(seeds, _, mappings)| {
            Almanac::new(seeds, mappings)
        })(input)
    }
}

pub(crate) fn parse(input: &str) -> miette::Result<models::Almanac, AocError> {
    common::nom(parser::parse_almanac, input)
}
