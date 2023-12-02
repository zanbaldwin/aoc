use crate::{aoc_error::AocError, parser::parse};

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let games = parse(input)?;
    let power: u32 = games
        .iter()
        .map(|game| game.get_minimum_counts())
        .map(|counts| counts.red as u32 * counts.green as u32 * counts.blue as u32)
        .sum();
    Ok(power.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input).unwrap());
    }
}
