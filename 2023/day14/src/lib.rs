#[cfg(test)]
pub(crate) mod display;
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

pub(crate) trait Spin {
    fn spin(&mut self, times: usize);
    fn turbo(&mut self, iterations: usize) -> bool;
}

pub(crate) mod models {
    use crate::{error::Error, Spin, Tilt, TiltDirection};
    use fasthash::{spooky::Hash64, RandomState};
    use std::{
        collections::{BTreeMap, HashMap},
        hash::Hash,
    };

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
    pub(crate) struct Position {
        pub(crate) x: usize,
        pub(crate) y: usize,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub(crate) enum Rock {
        Sphere,
        Cube,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
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
            input.lines().enumerate().try_for_each(|(y, line)| -> Result<(), Error> {
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
        fn tilt(&mut self, direction: &TiltDirection) {
            // Cannot mutate key, so we'll copy the result into another BTreeMap.
            let mut result: BTreeMap<Position, Rock> = BTreeMap::new();
            let mut dropspace: BTreeMap<usize, usize> = BTreeMap::new();
            let rocks = self.rocks.clone();

            // This required a lot of Googling and black magic to get this to
            // work, and I'm not sure I completely understand what it is doing.
            let mut rocks_iter = rocks.into_iter();
            let mut rev_iter;
            let rocks_iter: &mut dyn Iterator<Item = (Position, Rock)> = match direction {
                TiltDirection::North | TiltDirection::West => &mut rocks_iter,
                TiltDirection::South | TiltDirection::East => {
                    rev_iter = rocks_iter.rev();
                    &mut rev_iter
                },
            };

            rocks_iter.for_each(|(Position { x, y }, rock)| {
                let (position, rock) = match direction {
                    TiltDirection::North => {
                        let dropspace_for_column = dropspace.entry(x).or_insert(1);
                        match rock {
                            Rock::Cube => {
                                *dropspace_for_column = y + 1;
                                (Position { x, y }, rock)
                            },
                            Rock::Sphere => {
                                let y = *dropspace_for_column;
                                *dropspace_for_column = y + 1;
                                (Position { x, y }, rock)
                            },
                        }
                    },
                    TiltDirection::South => {
                        let dropspace_for_column = dropspace.entry(x).or_insert(self.height);
                        match rock {
                            Rock::Cube => {
                                *dropspace_for_column = y - 1;
                                (Position { x, y }, rock)
                            },
                            Rock::Sphere => {
                                let y = *dropspace_for_column;
                                *dropspace_for_column = y - 1;
                                (Position { x, y }, rock)
                            },
                        }
                    },
                    TiltDirection::West => {
                        let dropspace_for_column = dropspace.entry(y).or_insert(1);
                        match rock {
                            Rock::Cube => {
                                *dropspace_for_column = x + 1;
                                (Position { x, y }, rock)
                            },
                            Rock::Sphere => {
                                let x = *dropspace_for_column;
                                *dropspace_for_column = x + 1;
                                (Position { x, y }, rock)
                            },
                        }
                    },
                    TiltDirection::East => {
                        let dropspace_for_column = dropspace.entry(y).or_insert(self.width);
                        match rock {
                            Rock::Cube => {
                                *dropspace_for_column = x - 1;
                                (Position { x, y }, rock)
                            },
                            Rock::Sphere => {
                                let x = *dropspace_for_column;
                                *dropspace_for_column = x - 1;
                                (Position { x, y }, rock)
                            },
                        }
                    },
                };
                result.insert(position, rock);
            });
            self.rocks = result;
        }
    }
    impl Spin for Platform {
        fn spin(&mut self, times: usize) {
            let spin = [
                TiltDirection::North,
                TiltDirection::West,
                TiltDirection::South,
                TiltDirection::East,
            ];
            (1..=times).for_each(|_| spin.iter().for_each(|direction| self.tilt(direction)));
        }

        /// Turbo Mode!
        ///
        /// For spinning the platform an amount of times that would not take a
        /// reasonable amount of time (eg, 1 billion) then Turbo It!™
        ///
        /// Look, we all know that Eric chose 1 billion just to make sure we
        /// don't use a bruteforce approach. Look for a repeating pattern within
        /// a reasonable timeframe (which, depending on the number of iterations
        /// chosen, may still be an unreasonable amount of time).
        fn turbo(&mut self, iterations: usize) -> bool {
            // Change hashing algortihm: we don't care about being
            // cryptographically secure, nor do we care about HashDoS while
            // we're generating an answer. Change to something FAST!
            let mut spins = HashMap::with_hasher(RandomState::<Hash64>::new());
            // Kinda similar to checking if a number is prime, don't bother
            // checking for solutions greater than the square root of the number
            // you're looking for.
            for i in 1..=((iterations as f64).sqrt().floor() as usize) {
                self.spin(1);
                let state = self.clone();
                let matches: &mut Vec<usize> = spins.entry(state).or_default();
                if let Some(last) = matches.iter().last() {
                    // Pattern repeats every delta spins of the platform.
                    let delta = i - last;
                    // So, starting from the last time this pattern happened,
                    // will an integer number of deltas land on the iteration
                    // we're looking for?
                    if (iterations - last) % (delta) == 0 {
                        return true;
                    }
                }
                matches.push(i);
            }
            false
        }
    }
    impl Platform {
        pub(crate) fn total_load(&self) -> usize {
            self.rocks
                .iter()
                .filter(|(_position, rock)| rock == &&Rock::Sphere)
                .map(|(Position { y, .. }, _rock)| (*y as i32 - (self.height as i32 + 1)).unsigned_abs() as usize)
                .sum()
        }
    }
    impl Hash for Platform {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.rocks.hash(state)
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

        #[test]
        fn test_parse_and_display() {
            let platform: Platform = TEST_INPUT.try_into().unwrap();
            assert_eq!(TEST_INPUT.trim(), format!("{platform}").trim());
        }

        const NORTH_TILT: &str = "
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
        const SOUTH_TILT: &str = "
.....#....
....#....#
...O.##...
...#......
O.O....O#O
O.#..O.#.#
O....#....
OO....OO..
#OO..###..
#OO.O#...O";
        const EAST_TILT: &str = "
....O#....
.OOO#....#
.....##...
.OO#....OO
......OO#.
.O#...O#.#
....O#..OO
.........O
#....###..
#..OO#....
";
        const WEST_TILT: &str = "
O....#....
OOO.#....#
.....##...
OO.#OO....
OO......#.
O.#O...#.#
O....#OO..
O.........
#....###..
#OO..#....
";

        #[test]
        fn test_tilt_north() {
            let mut platform: Platform = TEST_INPUT.try_into().unwrap();
            let original = platform.clone();
            platform.tilt(&TiltDirection::North);
            assert_ne!(original, platform);
            println!("{platform}");
            assert_eq!(NORTH_TILT.trim(), format!("{platform}").trim())
        }

        #[test]
        fn test_tilt_south() {
            let mut platform: Platform = TEST_INPUT.try_into().unwrap();
            let original = platform.clone();
            platform.tilt(&TiltDirection::South);
            assert_ne!(original, platform);
            println!("{platform}");
            assert_eq!(SOUTH_TILT.trim(), format!("{platform}").trim())
        }

        #[test]
        fn test_tilt_east() {
            let mut platform: Platform = TEST_INPUT.try_into().unwrap();
            let original = platform.clone();
            platform.tilt(&TiltDirection::East);
            assert_ne!(original, platform);
            println!("{platform}");
            assert_eq!(EAST_TILT.trim(), format!("{platform}").trim())
        }

        #[test]
        fn test_tilt_west() {
            let mut platform: Platform = TEST_INPUT.try_into().unwrap();
            let original = platform.clone();
            platform.tilt(&TiltDirection::West);
            assert_ne!(original, platform);
            println!("{platform}");
            assert_eq!(WEST_TILT.trim(), format!("{platform}").trim())
        }
    }
}
