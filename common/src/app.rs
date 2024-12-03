use std::{fmt::Display, time::Duration};

use crate::{AdventOfCode, AocError, DayResult};
use clap::{Arg, Command};
use humanize_bytes::humanize_bytes_binary;
use humanize_duration::{prelude::DurationExt, Truncate};

type Response = Result<DayResult, AocError>;

pub struct Input {
    filepath: String,
    contents: String,
    elapsed: Duration,
    length: usize,
}
impl Input {
    pub(crate) fn new(filepath: String, contents: String, elapsed: Duration) -> Self {
        Self {
            filepath,
            length: contents.len(),
            contents,
            elapsed,
        }
    }

    pub(crate) fn contents(&self) -> &str {
        &self.contents
    }
}
impl Display for Input {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            r#"Input ({}) loaded from disk in {}.
Length: {}"#,
            self.filepath,
            self.elapsed.human(Truncate::Nano),
            humanize_bytes_binary!(self.length),
        )
    }
}

pub struct SingleYear {
    year: u16,
    cli: Command,
}
impl SingleYear {
    fn build_cli(year: u16) -> Command {
        Command::new("Advent of Code")
            .version("1.0")
            .author("Zan Baldwin <hello@zanbaldwin.com>")
            .about(format!("Solution Runner for AOC {}", year))
            .arg(
                Arg::new("day")
                    .help("Which day's solution to run?")
                    .required(true)
                    .index(1)
                    .value_parser(clap::value_parser!(u8)),
            )
    }

    pub fn new(year: u16) -> Self {
        Self { year, cli: Self::build_cli(year) }
    }

    pub fn run<P>(&mut self, proxy: P) -> i32
    where
        // Specify AdventOfCode directly, because `impl Runner` is too difficult.
        P: Fn(u8, Input) -> Response,
    {
        let matches = self.cli.clone().get_matches();
        let day: u8 = *matches.get_one::<u8>("day").expect("Day number is required to be a positive integer");
        println!("AOC {:04} (Day {:02})", self.year, day);
        println!("=================");

        let input = AdventOfCode::get_input(self.year, day).unwrap_or_else(|e| {
            eprintln!("Unable to fetch the input for {} Day {day}:", self.year);
            eprintln!("{e}");
            ::std::process::exit(2);
        });

        match proxy(day, input) {
            Ok(result) => {
                println!("{result}");
                0
            },
            Err(err) => {
                eprintln!("{err}");
                1
            },
        }
    }
}
