use common::Input;
use day03::{largest_number_with_digits, parse_banks};
use divan::Bencher;
use std::sync::OnceLock;

static INPUT: OnceLock<String> = OnceLock::new();
fn input() -> &'static str {
    INPUT.get_or_init(|| {
        Input::from_search(Some(env!("CARGO_PKG_NAME"))).expect("Could not find input file").into_string()
    })
}

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_parse() {
    let _ = parse_banks(input());
}

#[divan::bench]
fn bench_largest_number_2_digits(bencher: Bencher) {
    let banks = parse_banks(input()).unwrap();
    bencher.bench(|| banks.iter().map(|b| largest_number_with_digits(b, 2).unwrap()).sum::<u64>());
}

#[divan::bench]
fn bench_largest_number_12_digits(bencher: Bencher) {
    let banks = parse_banks(input()).unwrap();
    bencher.bench(|| banks.iter().map(|b| largest_number_with_digits(b, 12).unwrap()).sum::<u64>());
}

#[divan::bench]
fn bench_full_solve(bencher: Bencher) {
    bencher.bench(|| {
        let banks = parse_banks(input()).unwrap();
        let part1: u64 = banks.iter().map(|b| largest_number_with_digits(b, 2).unwrap()).sum();
        let part2: u64 = banks.iter().map(|b| largest_number_with_digits(b, 12).unwrap()).sum();
        (part1, part2)
    });
}
