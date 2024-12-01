use crate::{error::Error, models::Platform, Tilt, TiltDirection};

pub fn process(input: &str) -> Result<String, Error> {
    let mut platform: Platform = input.trim().try_into()?;
    platform.tilt(&TiltDirection::North);
    Ok(platform.total_load().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part1() {
        assert_eq!("136", process(TEST_INPUT).unwrap());
    }
}
