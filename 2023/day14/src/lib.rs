mod display;
pub mod error;
pub mod part1;
pub mod part2;

pub(crate) enum TiltDirection {
    North,
    East,
    South,
    West,
}
pub(crate) trait Tilt {
    fn tilt(&mut self, direction: &TiltDirection);
}

pub(crate) mod models {
    use crate::{error::Error, Tilt, TiltDirection};
    use std::collections::BTreeMap;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
    pub(crate) struct Position {
        pub(crate) x: usize,
        pub(crate) y: usize,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub(crate) enum Rock {
        Sphere,
        Cube,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub(crate) struct Platform {
        pub(crate) width: usize,
        pub(crate) height: usize,
        pub(crate) rocks: BTreeMap<Position, Rock>,
    }
    impl TryFrom<&str> for Platform {
        type Error = Error;
        fn try_from(input: &str) -> Result<Self, Self::Error> {
            let input = input.trim();
            let mut rocks = BTreeMap::new();
            let height: usize = input.lines().count();
            let mut width: Option<usize> = None;
            input
                .lines()
                .enumerate()
                .try_for_each(|(y, line)| -> Result<(), Error> {
                    let line_width = line.chars().count();
                    if width.get_or_insert(line_width) != &line_width {
                        return Err(Error::CouldNotDeterminePlatformWidth);
                    }

                    line.chars().enumerate().for_each(|(x, space)| {
                        let position = Position { x: x + 1, y: y + 1 };
                        match space {
                            'O' => rocks.insert(position, Rock::Sphere),
                            '#' => rocks.insert(position, Rock::Cube),
                            _ => None,
                        };
                    });

                    Ok(())
                })?;
            Ok(Self {
                width: width.ok_or(Error::CouldNotDeterminePlatformWidth)?,
                height,
                rocks,
            })
        }
    }
    impl Tilt for Platform {
        fn tilt(&mut self, _direction: &TiltDirection) {
            // Cannot mutate key, so we'll copy the result into another BTreeMap.
            let mut result: BTreeMap<Position, Rock> = BTreeMap::new();
            let mut dropspace: BTreeMap<usize, usize> = BTreeMap::new();
            let rocks = self.rocks.clone();
            rocks.into_iter().for_each(|(Position { x, y }, rock)| {
                let dropspace_for_column = dropspace.entry(x).or_insert(1);
                match rock {
                    Rock::Cube => {
                        *dropspace_for_column = y + 1;
                        result.insert(Position { x, y }, rock);
                    }
                    Rock::Sphere => {
                        let y = *dropspace_for_column;
                        *dropspace_for_column = y + 1;
                        result.insert(Position { x, y }, rock);
                    }
                }
            });
            self.rocks = result;
        }
    }
    impl Platform {
        pub(crate) fn total_load(&self) -> usize {
            self.rocks
                .iter()
                .filter(|(_position, rock)| rock == &&Rock::Sphere)
                .map(|(Position { y, .. }, _rock)| {
                    (*y as i32 - (self.height as i32 + 1)).abs() as usize
                })
                .sum()
        }
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
        const EXPECTED_OUTPUT: &str = "
OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

        #[test]
        fn test_parse_and_display() {
            let platform: Platform = TEST_INPUT.try_into().unwrap();
            assert_eq!(TEST_INPUT.trim(), format!("{platform}").trim());
        }

        #[test]
        fn test_tilt_and_print() {
            let mut platform: Platform = TEST_INPUT.try_into().unwrap();
            let original = platform.clone();
            platform.tilt(&TiltDirection::North);
            assert_ne!(original, platform);
            println!("{platform}");
            assert_eq!(EXPECTED_OUTPUT.trim(), format!("{platform}").trim())
        }
    }
}
