use crate::{aoc_error::AocError, parser::parse, EngineMap, Gear};

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let engine: EngineMap = parse(input)?.into();
    let sum_of_gear_ratios: usize =
        engine.get_gears().iter().map(|gear: &Gear| gear.1.iter().fold(1, |acc, part| acc * part.id)).sum();
    Ok(sum_of_gear_ratios.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("467835", process(input).unwrap());
    }
}
