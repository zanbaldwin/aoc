use crate::{error::Error, models::Platform, Spin};
pub fn process(input: &str, iterations: usize) -> Result<String, Error> {
    let mut platform: Platform = input.trim().try_into()?;

    if platform.turbo(iterations) {
        return Ok(platform.total_load().to_string());
    }

    Err(Error::NoSolutionInNormalTime)
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
        assert_eq!("64", process(TEST_INPUT, 1_000_000_000).unwrap());
    }

    const ONE_SPIN: &str = "
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";

    #[test]
    fn test_one_spin() {
        let mut platform: Platform = TEST_INPUT.try_into().unwrap();
        platform.spin(1);
        assert_eq!(ONE_SPIN.trim(), format!("{platform}").trim());
    }

    const TWO_SPIN: &str = "
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O";

    #[test]
    fn test_two_spin() {
        let mut platform: Platform = TEST_INPUT.try_into().unwrap();
        platform.spin(2);
        assert_eq!(TWO_SPIN.trim(), format!("{platform}").trim());
    }

    const THREE_SPIN: &str = "
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";

    #[test]
    fn test_three_spin() {
        let mut platform: Platform = TEST_INPUT.try_into().unwrap();
        platform.spin(3);
        assert_eq!(THREE_SPIN.trim(), format!("{platform}").trim());
    }
}
