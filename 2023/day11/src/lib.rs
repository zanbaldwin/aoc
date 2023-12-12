mod debug;
pub mod error;
pub mod part1;
pub mod part2;

mod models {
    use crate::error::Error;

    pub(crate) type Position = (usize, usize);

    #[derive(Debug, Copy, Clone)]
    pub(crate) struct Galaxy {
        pub(crate) id: usize,
        pub(crate) position: Position,
    }

    pub(crate) struct Universe {
        width: usize,
        height: usize,
        galaxies: Vec<Galaxy>,
    }
    impl TryFrom<&str> for Universe {
        type Error = Error;

        fn try_from(input: &str) -> Result<Self, Self::Error> {
            let mut galaxy_count: usize = 0;
            // Originally used a BTreeMap, but I want to mutate the keys later on, so
            // use a Vec instead.
            let mut galaxies: Vec<Galaxy> = Vec::new();
            let height: usize = input.lines().count();
            let mut width: Option<usize> = None;
            input
                .lines()
                .enumerate()
                .try_for_each(|(y, line)| -> Result<(), Error> {
                    let line_width = line.chars().count();
                    if width.get_or_insert(line_width) != &line_width {
                        return Err(Error::CouldNotDetermineUniverseWidth);
                    }
                    width.get_or_insert(line.chars().count());
                    line.chars().enumerate().for_each(|(x, c)| {
                        let position: Position = (x + 1, y + 1);
                        if c == '#' {
                            galaxy_count += 1;
                            galaxies.push(Galaxy {
                                position,
                                id: galaxy_count,
                            });
                        }
                    });
                    Ok(())
                })?;
            let width = width.ok_or(Error::CouldNotDetermineUniverseWidth)?;

            Ok(Self {
                width,
                height,
                galaxies,
            })
        }
    }
    impl Iterator for Universe {
        type Item = Galaxy;
        fn next(&mut self) -> Option<Self::Item> {
            self.galaxies.pop()
        }
    }
    impl Universe {
        pub(crate) fn expand(&self, growth: usize) -> Universe {
            let universe = self.clone();
            let mut rows_containing_no_galaxies: Vec<usize> = Vec::new();
            for line in 1..=universe.height {
                if universe
                    .iter()
                    .filter(
                        |Galaxy {
                             position: (_x, y), ..
                         }| y == &line,
                    )
                    .count()
                    == 0
                {
                    rows_containing_no_galaxies.push(line);
                }
            }

            let mut columns_containing_no_galaxies: Vec<usize> = Vec::new();
            for column in 1..=width {
                if galaxies
                    .iter()
                    .filter(
                        |Galaxy {
                             position: (x, _y), ..
                         }| x == &column,
                    )
                    .count()
                    == 0
                {
                    columns_containing_no_galaxies.push(column);
                }
            }

            columns_containing_no_galaxies.iter().for_each(|column| {
                galaxies
                    .iter_mut()
                    .filter(
                        |Galaxy {
                             position: (x, _y), ..
                         }| x > column,
                    )
                    .for_each(
                        |Galaxy {
                             position: (x, _y), ..
                         }| *x += 1,
                    );
            });
            rows_containing_no_galaxies.iter().for_each(|row| {
                galaxies
                    .iter_mut()
                    .filter(
                        |Galaxy {
                             position: (_x, y), ..
                         }| y > row,
                    )
                    .for_each(
                        |Galaxy {
                             position: (_x, y), ..
                         }| *y += 1,
                    );
            });
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const TEST_UNIVERSE: &str = "...#......
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
        fn test_try_from() {
            let universe: Universe = TEST_UNIVERSE.try_into().unwrap();
            assert_eq!(universe.width, 10);
            assert_eq!(universe.height, 10);
            assert_eq!(universe.galaxies.len(), 9);
        }

        #[test]
        fn test_expansion() {
            let universe: Universe = TEST_UNIVERSE.try_into().unwrap();
            let expanded_universe = universe.expand(1);
            let expected_universe: &str = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";
            assert_eq!(expected_universe, format!("{expanded_universe}"));
        }
    }
}
