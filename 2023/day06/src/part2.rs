use crate::parse_single;
use common::AocError;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let race = parse_single(input)?;
    // Definitely better ways to find this number than just collecting the range
    // and counting, but that requires editing other files or refactoring and we
    // don't do that here.
    let number_of_ways_to_win = race
        .get_beating_range()
        .expect("the race to be winnable")
        .into_iter()
        .count();
    Ok(number_of_ways_to_win.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY06_TEST_INPUT: &str = "Time:      7  15   30\nDistance:  9  40  200";

    #[test]
    fn test_part2() {
        assert_eq!("71503", process(DAY06_TEST_INPUT).unwrap());
    }
}
