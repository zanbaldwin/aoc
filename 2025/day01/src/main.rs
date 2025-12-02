use common::Input;
use day01::{count_zeros_landed, count_zeros_touched, models::InstructionSet, STARTING_POSITION};

fn main() {
    let input = Input::from_cli().expect("Failed to open input file").into_string();
    let instructions: InstructionSet = input.into();
    let spins = instructions.spin(STARTING_POSITION);
    println!("Part 1: {}", count_zeros_landed(&spins));
    println!("Part 2: {}", count_zeros_touched(&spins));
}
