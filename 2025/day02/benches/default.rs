use day02::parse;
use divan::Bencher;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_parse() {
    let _ = parse(INPUT);
}

#[divan::bench]
fn bench_repeated_twice_ids(bencher: Bencher) {
    let pairs = parse(INPUT);
    bencher.bench(|| pairs.iter().flat_map(|p| p.repeated_twice_ids()).count());
}

#[divan::bench]
fn bench_repeated_any_ids(bencher: Bencher) {
    let pairs = parse(INPUT);
    bencher.bench(|| pairs.iter().flat_map(|p| p.repeated_any_ids()).count());
}

#[divan::bench]
fn bench_full_solve(bencher: Bencher) {
    bencher.bench(|| {
        let pairs = parse(INPUT);
        let part1: u64 = pairs.iter().flat_map(|p| p.repeated_twice_ids()).sum();
        let part2: u64 = pairs.iter().flat_map(|p| p.repeated_any_ids()).sum();
        (part1, part2)
    });
}
