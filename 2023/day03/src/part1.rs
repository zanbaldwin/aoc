use crate::{aoc_error::AocError, parser::parse, Engine, EngineMap};

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let engine: Engine = parse(input)?;
    let map: EngineMap = engine.into();
    let parts = map.get_parts_neighbouring_any_symbol();
    let total: usize = parts.iter().map(|part| part.id).sum();
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
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
        assert_eq!("4361", process(input).unwrap());
    }
}
