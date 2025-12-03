use common::input;
use day03::{Error, largest_number_with_digits, parse_banks};

fn main() {
    let input = input!().expect("input error").into_string();

    let banks = parse_banks(&input).unwrap();
    println!(
        "Part 1: {}",
        banks
            .iter()
            .map(|b| largest_number_with_digits(b, 2))
            .collect::<Result<Vec<_>, Error>>()
            .expect("Input contained a battery bank that did not contain enough batteries")
            .iter()
            .sum::<u64>()
    );
    println!(
        "Part 2: {}",
        banks
            .iter()
            .map(|b| largest_number_with_digits(b, 12))
            .collect::<Result<Vec<_>, Error>>()
            .expect("Input contained a battery bank that did not contain enough batteries")
            .iter()
            .sum::<u64>()
    );
}
