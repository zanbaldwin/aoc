use common::{Solution, input};

fn main() {
    let input = input!().expect("input error");
    let solution = day06::Day06::parse(input).unwrap();
    println!("Part 1: {}", solution.part1().unwrap());
    println!("Part 2: {}", solution.part2().unwrap());
}
