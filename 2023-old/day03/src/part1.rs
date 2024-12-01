use crate::{aoc_error::AocError, parser::parse, EngineMap};

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let engine: EngineMap = parse(input)?.into();
    let sum_of_valid_part_numbers: usize = engine
        .get_parts_neighbouring_any_symbol()
        .iter()
        .map(|part| part.id)
        .sum();
    Ok(sum_of_valid_part_numbers.to_string())
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
