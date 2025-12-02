use crate::ROTARY_POSITIONS;
use std::ops::{Deref, Div};

#[derive(Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone)]
pub struct Instruction {
    direction: Direction,
    clicks: u16,
}
impl<T: AsRef<str>> From<T> for Instruction {
    fn from(value: T) -> Self {
        let line = value.as_ref().trim();
        let direction = match line.chars().nth(0) {
            Some('L') => Direction::Left,
            Some('R') => Direction::Right,
            _ => panic!("Invalid input, dumbass."),
        };
        let clicks: u16 = line[1..].parse().expect("Invalid input, dumbass.");
        Self { direction, clicks }
    }
}
impl Instruction {
    fn spin(&self, start: u8) -> Spin {
        Spin::apply_instruction(self.clone(), start)
    }
}

pub struct InstructionSet {
    instructions: Vec<Instruction>,
}
impl<T: AsRef<str>> From<T> for InstructionSet {
    fn from(value: T) -> Self {
        let instructions: Vec<Instruction> = value.as_ref().trim().lines().map(|l| l.into()).collect();
        Self { instructions }
    }
}
impl Deref for InstructionSet {
    type Target = Vec<Instruction>;
    fn deref(&self) -> &Self::Target {
        &self.instructions
    }
}
impl InstructionSet {
    fn len(&self) -> usize {
        self.instructions.len()
    }

    pub fn spin(&self, start: u8) -> Vec<Spin> {
        let mut spins: Vec<Spin> = Vec::with_capacity(self.len() + 1);
        let mut position = start;
        for instruction in self.iter() {
            let spin = instruction.spin(position);
            position = spin.land();
            spins.push(spin)
        }
        spins
    }
}

pub struct Spin {
    start: u8,
    instruction: Instruction,
}
impl Spin {
    pub(crate) fn apply_instruction(instruction: Instruction, start: u8) -> Self {
        Self { start, instruction }
    }

    // Final position after applying instruction.
    pub(crate) fn land(&self) -> u8 {
        let clicks = match self.instruction.direction {
            Direction::Left => -(self.instruction.clicks as i32),
            Direction::Right => self.instruction.clicks as i32,
        };
        (self.start as i32 + clicks).rem_euclid(ROTARY_POSITIONS as i32) as u8
    }

    /// Maximum number of wraps for a dial with 100 positions, and u16 clicks: 655 (fits inside a u16 return value).
    pub(crate) fn wraps(&self) -> u16 {
        let start = u16::from(self.start);
        let total_positions = u16::from(ROTARY_POSITIONS);
        let clicks_until_zero = match self.instruction.direction {
            Direction::Left => match start {
                0 => total_positions,
                _ => start,
            },
            Direction::Right => total_positions.saturating_sub(start),
        };
        if self.instruction.clicks >= clicks_until_zero {
            let start_on_zero = self.start == 0;
            let remaining_clicks = self.instruction.clicks.saturating_sub(clicks_until_zero);
            let whole_rotations = remaining_clicks.rem_euclid(total_positions) == 0;
            let modifier = if !start_on_zero && whole_rotations { 0 } else { 1 };
            let passes = self.instruction.clicks.saturating_sub(clicks_until_zero).div(total_positions);
            passes + modifier
        } else {
            0
        }
    }
}

mod debug {
    use super::*;
    use std::fmt::Display;

    impl Display for Direction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Self::Left => "L",
                    Self::Right => "R",
                }
            )
        }
    }
    impl Display for Instruction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}{}", self.direction, self.clicks)
        }
    }
    impl Display for InstructionSet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let set: Vec<_> = self.instructions.iter().map(|i| i.to_string()).collect();
            write!(f, "{}", set.join("\n"))
        }
    }
}
