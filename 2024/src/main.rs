use aoc_2024::day01;
use aoc_common::{app::SingleYear as Application, AdventOfCode, AocError};
use humanize_duration::{prelude::DurationExt, Truncate};
use std::{process::exit, time::Instant};

fn main() -> ! {
    let now = Instant::now();
    let code = Application::new(2024).run(|day: u8, input: &str| match day {
        // Construct and run each day's solution manually, as they all use a
        // different associated type on the Solution trait they implement.
        1 => AdventOfCode::run(day01::Day01 {}, input),
        _ => Err(AocError::OutOfScope(day)),
    });
    println!("Total: {}", now.elapsed().human(Truncate::Nano));
    exit(code);
}
