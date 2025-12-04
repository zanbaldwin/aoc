use common::Input;
use common::input::FileInput;
use day01::models::InstructionSet;
use day01::{STARTING_POSITION, count_zeros_landed, count_zeros_touched};
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
    let _ = Into::<InstructionSet>::into(input());
}

#[divan::bench]
fn bench_spin(bencher: Bencher) {
    let instructions: InstructionSet = input().into();
    bencher.bench(|| instructions.spin(STARTING_POSITION));
}

#[divan::bench]
fn bench_count_zeros_landed(bencher: Bencher) {
    let instructions: InstructionSet = input().into();
    let spins = instructions.spin(STARTING_POSITION);
    bencher.bench(|| count_zeros_landed(&spins));
}

#[divan::bench]
fn bench_count_zeros_touched(bencher: Bencher) {
    let instructions: InstructionSet = input().into();
    let spins = instructions.spin(STARTING_POSITION);
    bencher.bench(|| count_zeros_touched(&spins));
}

#[divan::bench]
fn bench_part1_solve(bencher: Bencher) {
    bencher.bench(|| {
        let instructions: InstructionSet = input().into();
        let spins = instructions.spin(STARTING_POSITION);
        count_zeros_landed(&spins)
    });
}

#[divan::bench]
fn bench_part2_solve(bencher: Bencher) {
    bencher.bench(|| {
        let instructions: InstructionSet = input().into();
        let spins = instructions.spin(STARTING_POSITION);
        count_zeros_touched(&spins)
    });
}

#[divan::bench]
fn bench_full_solve(bencher: Bencher) {
    bencher.bench(|| {
        let instructions: InstructionSet = input().into();
        let spins = instructions.spin(STARTING_POSITION);
        let part1 = count_zeros_landed(&spins);
        let part2 = count_zeros_touched(&spins);
        (part1, part2)
    });
}
