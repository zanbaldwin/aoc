use crate::{error::Error, models::Universe};

pub fn process(input: &str) -> Result<String, Error> {
    let universe: Universe = input.try_into()?;
    let expanded = universe.measure().expand(1);
    let distances = expanded.distances();
    let total: usize = distances.into_iter().map(|(_, distance)| distance).sum();
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const GALACTICAL_MAP: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part1() {
        assert_eq!("374", process(GALACTICAL_MAP).unwrap());
    }

    #[test]
    fn test_expansion() {}
}
