pub mod app;
mod error;

pub use error::*;
use humanize_duration::{prelude::DurationExt, Truncate};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs;
use std::io;
use std::path::Path;
use std::time::{Duration, Instant};

pub trait Solution {
    type Parsed;
    fn day(&self) -> u8;
    fn parse(&self, input: &str) -> Result<Self::Parsed, ParseError>;
    fn part1(&self, input: &Self::Parsed) -> Result<impl Display, RunnerError>;
    fn part2(&self, input: &Self::Parsed) -> Result<impl Display, RunnerError>;
}

pub struct AdventOfCode {}
impl AdventOfCode {
    fn get_input(year: u16, day: u8) -> io::Result<String> {
        let target_path = Path::new(file!())
            .parent()
            .ok_or(io::Error::new(io::ErrorKind::Other, "Failed to get the parent directory of the source file"))?
            // Input files relative to this source file.
            .join(format!("../../input/{}/{:02}.txt", year, day));
        fs::read_to_string(fs::canonicalize(&target_path).unwrap_or(target_path))
    }

    pub fn run<S: Solution>(day: S, input: &str) -> Result<DayResult, AocError> {
        let now = Instant::now();
        let parsed = day.parse(input)?;
        let parse_time = now.elapsed();

        let now = Instant::now();
        let part_one_result = day.part1(&parsed);
        let part_one_time = now.elapsed();

        let now = Instant::now();
        let part_two_result = day.part2(&parsed);
        let part_two_time = now.elapsed();

        Ok(DayResult {
            parse_time,
            part_one: PartResult {
                time: part_one_time,
                answer: part_one_result
                    .map(|result| RunResult::Success(result.to_string()))
                    .unwrap_or_else(RunResult::Fail),
            },
            part_two: PartResult {
                time: part_two_time,
                answer: part_two_result
                    .map(|result| RunResult::Success(result.to_string()))
                    .unwrap_or_else(RunResult::Fail),
            },
        })
    }
}

enum RunResult {
    Success(String),
    Fail(RunnerError),
}
struct PartResult {
    answer: RunResult,
    time: Duration,
}
pub struct DayResult {
    parse_time: Duration,
    part_one: PartResult,
    part_two: PartResult,
}
impl Display for DayResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(
            f,
            r#"Parsed: in {}

- Part 1: {}
   ... in {} ({})
- Part 2: {}
   ... in {} ({})"#,
            self.parse_time.human(Truncate::Nano),
            match &self.part_one.answer {
                RunResult::Success(result) => result,
                RunResult::Fail(_err) => "fuck a duck! an error occured!",
            },
            self.part_one.time.human(Truncate::Nano),
            (self.part_one.time + self.parse_time).human(Truncate::Nano),
            match &self.part_two.answer {
                RunResult::Success(result) => result,
                RunResult::Fail(_err) => "fuck a duck! an error occured!",
            },
            self.part_two.time.human(Truncate::Nano),
            (self.part_two.time + self.parse_time).human(Truncate::Nano),
        )
    }
}
