use crate::{aoc_error::AocError, parser::parse, Engine, EngineMap, Gear};

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let engine: Engine = parse(input)?;
    let map: EngineMap = engine.into();
    let gears: Vec<Gear> = map.get_gears();
    let total: usize = gears
        .iter()
        .map(|gear: &Gear| gear.1.iter().fold(1, |acc, part| acc * part.id))
        .sum();
    Ok(total.to_string())
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
