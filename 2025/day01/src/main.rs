use common::{Input, input};
use day01::{STARTING_POSITION, count_zeros_landed, count_zeros_touched, models::InstructionSet};

fn main() {
    let input = input!().expect("input error");
    let instructions: InstructionSet = input.as_str().into();
    let spins = instructions.spin(STARTING_POSITION);
    println!("Part 1: {}", count_zeros_landed(&spins));
    println!("Part 2: {}", count_zeros_touched(&spins));
}
