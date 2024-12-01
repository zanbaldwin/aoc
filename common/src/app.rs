use std::time::Instant;

use crate::{AdventOfCode, AocError, DayResult};
use clap::{Arg, Command};
use humanize_bytes::humanize_bytes_binary;
use humanize_duration::prelude::DurationExt;
use humanize_duration::Truncate;

type Response = Result<DayResult, AocError>;

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
        P: Fn(u8, &str) -> Response,
    {
        let matches = self.cli.clone().get_matches();
        println!("AOC {:04}", self.year);
        println!("========");
        let day: u8 = *matches.get_one::<u8>("day").expect("Day number is required to be a positive integer");
        let now = Instant::now();
        let input = AdventOfCode::get_input(self.year, day);
        let elapsed = now.elapsed();
        let input = input.map_err(AocError::Io).unwrap_or_else(|e| {
            eprintln!("{e}");
            ::std::process::exit(2);
        });
        println!("Input: \"input/{}/{:02}.txt\"", self.year, day);
        println!("Loaded from disk in: {}", elapsed.human(Truncate::Nano));
        println!("Length: {}", humanize_bytes_binary!(input.len()));
        match proxy(day, &input) {
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
