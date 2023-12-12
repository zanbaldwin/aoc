use crate::{error::Error, models::Universe};

pub fn process(input: &str) -> Result<String, Error> {
    let universe: Universe = input.try_into()?;
    let expanded = universe.measure().expand(1_000_000);
    let distances = expanded.distances();
    let total: usize = distances.into_values().sum();
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
    fn test_part1_ten() {
        let universe: Universe = GALACTICAL_MAP.try_into().unwrap();
        let expanded = universe.measure().expand(10);
        println!("{expanded}");
        let total: usize = expanded
            .distances()
            .into_iter()
            .map(|(_, distance)| distance)
            .sum();
        assert_eq!(1030, total);
    }

    #[test]
    fn test_part1_hundred() {
        let universe: Universe = GALACTICAL_MAP.try_into().unwrap();
        let expanded = universe.measure().expand(100);
        let total: usize = expanded
            .distances()
            .into_iter()
            .map(|(_, distance)| distance)
            .sum();
        assert_eq!(8410, total);
    }
}
