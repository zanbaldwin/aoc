use common::Input;
use day02::parse;
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
    let _ = parse(input());
}

#[divan::bench]
fn bench_repeated_twice_ids(bencher: Bencher) {
    let pairs = parse(input());
    bencher.bench(|| pairs.iter().flat_map(|p| p.repeated_twice_ids()).count());
}

#[divan::bench]
fn bench_repeated_any_ids(bencher: Bencher) {
    let pairs = parse(input());
    bencher.bench(|| pairs.iter().flat_map(|p| p.repeated_any_ids()).count());
}

#[divan::bench]
fn bench_full_solve(bencher: Bencher) {
    bencher.bench(|| {
        let pairs = parse(input());
        let part1: u64 = pairs.iter().flat_map(|p| p.repeated_twice_ids()).sum();
        let part2: u64 = pairs.iter().flat_map(|p| p.repeated_any_ids()).sum();
        (part1, part2)
    });
}
