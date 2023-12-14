use crate::{error::Error, models::Platform, Tilt, TiltDirection};

pub fn process(input: &str) -> Result<String, Error> {
    let mut platform: Platform = input.trim().try_into()?;

    [
        TiltDirection::North,
        TiltDirection::West,
        TiltDirection::South,
        TiltDirection::East,
    ]
    .iter()
    .cycle()
    .take(1_000)
    .for_each(|direction| platform.tilt(direction));

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
    fn test_part2() {
        assert_eq!("64", process(TEST_INPUT).unwrap());
    }
}
