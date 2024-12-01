use aoc_2024::{day00::Day00, day01::Day01, day02::Day02};
use aoc_common::{app::SingleYear as Application, AdventOfCode, AocError};
use std::process::exit;

fn main() -> ! {
    let code = Application::new(2024).run(|day: u8, input: &str| match day {
        // Manually construct and run each day's solution,
        // because they use a different Solution::Parsed type.
        0 => AdventOfCode::run(Day00 {}, input),
        1 => AdventOfCode::run(Day01 {}, input),
        2 => AdventOfCode::run(Day02 {}, input),
        _ => Err(AocError::OutOfScope(day)),
    });
    exit(code);
}
