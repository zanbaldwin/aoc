use common::AocError;

pub mod part1;
pub mod part2;

type Time = u64;
type Distance = u64;

mod models {
    use super::*;
    use std::ops::RangeInclusive;

    pub(crate) struct Race {
        pub(crate) time: Time,
        pub(crate) distance_to_beat: Distance,
    }
    impl Race {
        pub fn new(time: Time, distance_to_beat: Distance) -> Self {
            Self {
                time,
                distance_to_beat,
            }
        }

        fn distance_after_warmup(&self, warmup: Time) -> bool {
            let distance = if warmup >= self.time {
                0
            } else {
                warmup * (self.time - warmup)
            };
            distance > self.distance_to_beat
        }

        pub fn get_beating_range(&self) -> Option<RangeInclusive<Time>> {
            // Non-inclusive range starting from 1 (because both 0 and total
            // race time results in a distance of zero).
            let min = match (1..self.time).find(|warmup: &Time| self.distance_after_warmup(*warmup))
            {
                Some(min) => min,
                None => {
                    return None;
                }
            };

            let max = match (1..self.time)
                .rev()
                .find(|warmup: &Time| self.distance_after_warmup(*warmup))
            {
                Some(max) => max,
                None => panic!("Upper limit not found after calculated minimum."),
            };

            Some(min..=max)
        }
    }
}

mod parser {
    use super::models::*;
    use nom::{
        bytes::complete::tag,
        character::complete::{line_ending, space1, u64},
        combinator::map,
        multi::separated_list1,
        sequence::{preceded, separated_pair, terminated, tuple},
        IResult,
    };

    pub(crate) fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
        map(
            separated_pair(
                parse_number_list("Time"),
                line_ending,
                parse_number_list("Distance"),
            ),
            |(times, distances)| {
                assert_eq!(times.len(), distances.len());
                times
                    .into_iter()
                    .zip(distances)
                    .map(|(time, distance)| Race::new(time, distance))
                    .collect()
            },
        )(input)
    }

    fn parse_number_list<'a>(
        list_name: &str,
    ) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<u64>> + '_ {
        move |input: &'a str| {
            preceded(
                terminated(tuple((tag(list_name), tag(":"))), space1),
                separated_list1(space1, u64),
            )(input)
        }
    }
}

pub(crate) fn parse(input: &str) -> miette::Result<Vec<models::Race>, AocError> {
    common::nom(parser::parse_races, input)
}
