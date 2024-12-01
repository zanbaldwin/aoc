use aoc_common::{ParseError, RunnerError, Solution};
use models::Race;
use std::fmt::Display;

pub struct Day06 {}
impl Solution for Day06 {
    type Parsed = Vec<models::Race>;

    fn parse(&self, input: &str) -> Result<Self::Parsed, ParseError> {
        parser::parse(input)
    }

    fn part1(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        Ok(input
            .into_iter()
            .filter_map(|race| race.get_beating_range())
            // `.len()` is not available on Range<u64> because the number could be
            // larger than what 32bit systems can hold. But `.into_iter().count()`
            // should get the job done just as well.
            .map(|range| range.into_iter().count())
            .product::<usize>())
    }

    fn part2(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        let mut time = String::new();
        let mut record = String::new();
        input.into_iter().for_each(|race| {
            time.push_str(race.time.to_string().as_str());
            record.push_str(race.record.to_string().as_str());
        });
        let race = Race::new(
            time.parse().map_err(|_| RunnerError::Custom("Time ParseIntError".to_string()))?,
            record.parse().map_err(|_| RunnerError::Custom("Record ParseIntError".to_string()))?,
        );

        Ok(race
            .get_beating_range()
            .ok_or_else(|| RunnerError::Custom("Couldn't determine beating range".to_string()))?
            .into_iter()
            .count())
    }
}

mod models {
    use std::ops::RangeInclusive;

    type Time = u64;
    type Distance = u64;

    #[derive(Clone, Debug, PartialEq)]
    pub struct Race {
        pub(crate) time: Time,
        pub(crate) record: Distance,
    }
    impl Race {
        pub(crate) fn new(time: Time, record: Distance) -> Self {
            Self { time, record }
        }

        fn distance_after_warmup(&self, warmup: Time) -> bool {
            let distance = warmup * (self.time - warmup);
            distance > self.record
        }

        pub(crate) fn get_beating_range(&self) -> Option<RangeInclusive<Time>> {
            // Non-inclusive range starting from 1 (because both 0 and total
            // race time results in a distance of zero).
            let min = match (1..self.time).find(|warmup: &Time| self.distance_after_warmup(*warmup)) {
                Some(min) => min,
                None => {
                    return None;
                },
            };
            let max = (1..self.time).rev().find(|warmup: &Time| self.distance_after_warmup(*warmup))?;
            Some(min..=max)
        }
    }
}

mod parser {
    use super::models::*;
    use aoc_common::ParseError;
    use nom::{
        bytes::complete::tag,
        character::complete::{line_ending, space0, space1, u64},
        combinator::map,
        multi::separated_list1,
        sequence::{preceded, separated_pair, tuple},
        IResult,
    };

    fn parse_number_list<'a>(list_name: &str) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<u64>> + '_ {
        move |input: &'a str| {
            preceded(tuple((tag(list_name), space0, tag(":"), space1)), separated_list1(space1, u64))(input)
        }
    }

    fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
        map(
            separated_pair(parse_number_list("Time"), line_ending, parse_number_list("Distance")),
            |(times, distances)| {
                assert_eq!(times.len(), distances.len());
                times.into_iter().zip(distances).map(|(time, record)| Race { time, record }).collect()
            },
        )(input)
    }

    pub(crate) fn parse(input: &str) -> Result<Vec<Race>, ParseError> {
        parse_races(input).map_err(|e| ParseError::Nom(e.to_string())).map(|(_, races)| races)
    }
}

#[cfg(test)]
mod tests {
    use super::models::*;
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_parse() {
        let solution = Day06 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!(vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)], parsed);
    }

    #[test]
    fn test_part_one() {
        let solution = Day06 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!("288", solution.part1(parsed).unwrap().to_string());
    }

    #[test]
    fn test_part_two() {
        let solution = Day06 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!("71503", solution.part2(parsed).unwrap().to_string());
    }
}
