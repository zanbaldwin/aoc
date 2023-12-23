use crate::parser::parse_race;
use common::AocError;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let race = parse_race(input)?;
    let number_of_ways_to_win = race.get_beating_range().expect("the race to be winnable").count();
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
