use day01::{count_zeros_landed, count_zeros_touched, models::InstructionSet, STARTING_POSITION};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let instructions: InstructionSet = INPUT.into();
    let spins = instructions.spin(STARTING_POSITION);
    println!("Part 1: {}", count_zeros_landed(&spins));
    println!("Part 2: {}", count_zeros_touched(&spins));
}
