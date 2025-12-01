pub mod models;
use models::*;

const ROTARY_POSITIONS: u8 = 100;
pub const STARTING_POSITION: u8 = 50;

pub fn count_zeros_landed(spins: &[Spin]) -> usize {
    spins.iter().filter(|spin| spin.land() == 0).count()
}

pub(crate) fn count_zeros_wrapped(spins: &[Spin]) -> usize {
    spins.iter().map(|spin| usize::from(spin.wraps())).sum()
}

pub fn count_zeros_touched(spins: &[Spin]) -> usize {
    count_zeros_wrapped(spins) + count_zeros_landed(spins)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    const TEST_INPUT: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    #[test]
    fn test_parse_and_print() {
        let instructions: InstructionSet = TEST_INPUT.into();
        assert_eq!(TEST_INPUT.trim(), instructions.to_string().as_str());
    }

    #[test]
    fn test_rotations() {
        let instructions: InstructionSet = TEST_INPUT.into();
        let expected = vec![
            /* Omit starting position: 50, */
            82, 52, 0, 95, 55, 0, 99, 0, 14, 32,
        ];
        assert_eq!(expected, instructions.spin(STARTING_POSITION).iter().map(|spin| spin.land()).collect::<Vec<_>>());
    }

    #[test]
    fn test_zeros_landed() {
        let instructions: InstructionSet = TEST_INPUT.into();
        let spins = instructions.spin(STARTING_POSITION);
        assert_eq!(3, count_zeros_landed(&spins));
    }

    #[rstest]
    #[case(10, "L9", 0, 1)]
    // Landing on zero is considered different to passing over zero.
    #[case(10, "L10", 0, 0)]
    #[case(9, "L10", 1, 99)]
    #[case(9, "L11", 1, 98)]
    #[case(10, "L215", 3, 95)]
    // If it starts at zero, it shouldn't wrap unless it touches zero AGAIN.
    #[case(0, "L99", 0, 1)]
    #[case(0, "L100", 1, 0)]
    #[case(0, "L101", 1, 99)]
    #[case(90, "R9", 0, 99)]
    // Landing on zero is considered different to passing over zero.
    #[case(90, "R10", 0, 0)]
    #[case(91, "R10", 1, 1)]
    #[case(90, "R11", 1, 1)]
    #[case(90, "R215", 3, 5)]
    // If it starts at zero, it shouldn't wrap unless it touches zero AGAIN.
    #[case(0, "R99", 0, 99)]
    #[case(0, "R100", 1, 0)]
    #[case(0, "R101", 1, 1)]
    // From the example
    #[case(95, "R60", 1, 55)]
    #[case(50, "R1000", 10, 50)]
    // Stupid fucking bug that wasn't caught by any unit tests (landing on zero doesn't count as a crossing):
    #[case(20, "R180", 1, 0)]
    fn test_wrapping_count(#[case] start: u8, #[case] instruction: &str, #[case] wraps: u16, #[case] land: u8) {
        let spin = Spin::apply_instruction(instruction.into(), start);
        assert_eq!(wraps, spin.wraps());
        assert_eq!(land, spin.land());
    }

    #[test]
    fn test_zeros_wrapped() {
        let instructions: InstructionSet = TEST_INPUT.into();
        let spins = instructions.spin(STARTING_POSITION);
        assert_eq!(3, count_zeros_wrapped(&spins));
    }

    #[test]
    fn test_zeros_touched() {
        let instructions: InstructionSet = TEST_INPUT.into();
        let spins = instructions.spin(STARTING_POSITION);
        assert_eq!(6, count_zeros_touched(&spins));
    }
}
