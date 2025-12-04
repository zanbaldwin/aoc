use common::{Input, input};
use day02::parse;

fn main() {
    let input = input!().expect("input error");
    let pairs = parse(input.as_str());
    let part1: u64 = pairs.iter().flat_map(|p| p.repeated_twice_ids()).sum();
    let part2: u64 = pairs.iter().flat_map(|p| p.repeated_any_ids()).sum();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
