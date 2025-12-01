use day01::models::InstructionSet;
use day01::{count_zeros_landed, count_zeros_touched, STARTING_POSITION};
use divan::Bencher;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_parse() {
    let _ = Into::<InstructionSet>::into(INPUT);
}

#[divan::bench]
fn bench_spin(bencher: Bencher) {
    let instructions: InstructionSet = INPUT.into();
    bencher.bench(|| instructions.spin(STARTING_POSITION));
}

#[divan::bench]
fn bench_count_zeros_landed(bencher: Bencher) {
    let instructions: InstructionSet = INPUT.into();
    let spins = instructions.spin(STARTING_POSITION);
    bencher.bench(|| count_zeros_landed(&spins));
}

#[divan::bench]
fn bench_count_zeros_touched(bencher: Bencher) {
    let instructions: InstructionSet = INPUT.into();
    let spins = instructions.spin(STARTING_POSITION);
    bencher.bench(|| count_zeros_touched(&spins));
}

#[divan::bench]
fn bench_part1_solve(bencher: Bencher) {
    bencher.bench(|| {
        let instructions: InstructionSet = INPUT.into();
        let spins = instructions.spin(STARTING_POSITION);
        let part1 = count_zeros_landed(&spins);
        part1
    });
}

#[divan::bench]
fn bench_part2_solve(bencher: Bencher) {
    bencher.bench(|| {
        let instructions: InstructionSet = INPUT.into();
        let spins = instructions.spin(STARTING_POSITION);
        let part2 = count_zeros_touched(&spins);
        part2
    });
}

#[divan::bench]
fn bench_full_solve(bencher: Bencher) {
    bencher.bench(|| {
        let instructions: InstructionSet = INPUT.into();
        let spins = instructions.spin(STARTING_POSITION);
        let part1 = count_zeros_landed(&spins);
        let part2 = count_zeros_touched(&spins);
        (part1, part2)
    });
}
