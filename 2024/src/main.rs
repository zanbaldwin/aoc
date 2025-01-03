use aoc_2024::{day01, day02, day03, day04};
use aoc_common::{
    app::{Input, SingleYear as Application},
    AdventOfCode, AocError,
};
use humanize_duration::{prelude::DurationExt, Truncate};
use std::{process::exit, time::Instant};

fn main() -> ! {
    let now = Instant::now();
    let code = Application::new(2024).run(|day: u8, input: Input| match day {
        // Construct and run each day's solution manually, as they all use a
        // different associated type on the Solution trait they implement.
        1 => AdventOfCode::run(day01::Day01 {}, input),
        2 => AdventOfCode::run(day02::Day02 {}, input),
        3 => AdventOfCode::run(day03::Day03 {}, input),
        4 => AdventOfCode::run(day04::Day04 {}, input),
        _ => Err(AocError::OutOfScope(day)),
    });
    println!("Total: {}", now.elapsed().human(Truncate::Nano));
    exit(code);
}
