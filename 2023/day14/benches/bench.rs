use day14::{part1, part2};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(INPUT)).unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(INPUT), 1_000_000_000).unwrap();
}
