use common::{
    Input, Solution,
    input::{FileInput, RawInput},
};
use day04::Day04;
use divan::Bencher;
use std::sync::OnceLock;

static INPUT: OnceLock<String> = OnceLock::new();
fn input() -> &'static str {
    INPUT.get_or_init(|| {
        FileInput::from_search(Some(env!("CARGO_PKG_NAME"))).expect("Could not find input file").into_string()
    })
}

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_parse() {
    // TODO: Figure out a _pretty_ way of passing input without unnecessary string allocations.
    let _ = Day04::from_input(RawInput::new(input()));
}

#[divan::bench]
fn bench_part1(bencher: Bencher) {
    let solution = Day04::from_input(RawInput::new(input())).unwrap();
    bencher.bench(|| solution.part1().unwrap());
}

#[divan::bench]
fn bench_part2(bencher: Bencher) {
    let solution = Day04::from_input(RawInput::new(input())).unwrap();
    bencher.bench(|| solution.part2().unwrap());
}

#[divan::bench]
fn bench_full_solve(bencher: Bencher) {
    bencher.bench(|| {
        let solution = Day04::from_input(RawInput::new(input())).unwrap();
        let part1 = solution.part1().unwrap();
        let part2 = solution.part2().unwrap();
        (part1, part2)
    });
}
