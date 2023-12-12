mod display;
pub mod error;
pub mod part1;
pub mod part2;

pub(crate) mod models {
    use crate::error::Error;
    use itertools::Itertools;
    use std::{
        cmp::{max, min},
        collections::BTreeMap,
    };

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
    pub(crate) struct Position {
        pub(crate) x: usize,
        pub(crate) y: usize,
    }
    impl Position {
        fn distance_from(&self, other: Self) -> usize {
            let max_x = max(self.x, other.x);
            let min_x = min(self.x, other.x);
            let delta_x = max_x - min_x;

            let max_y = max(self.y, other.y);
            let min_y = min(self.y, other.y);
            let delta_y = max_y - min_y;

            delta_x + delta_y
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub(crate) struct Galaxy {
        pub(crate) id: usize,
        pub(crate) position: Position,
        pub(crate) shift: Position,
    }
    impl Galaxy {
        pub(crate) fn resolve(&mut self) {
            self.position.x += self.shift.x;
            self.position.y += self.shift.y;
            self.shift = Position::default();
        }
    }

    pub(crate) struct Spacing<'a> {
        pub(crate) universe: &'a Universe,
        pub(crate) rows: Vec<usize>,
        pub(crate) columns: Vec<usize>,
    }
    impl<'a> Spacing<'a> {
        pub(crate) fn expand(&self, exponential: usize) -> Universe {
            // Exponential is how many times wider the gap between galaxies
            // should be. So, starting from single lines, doubling (2) should
            // add 1 line and tripling (3) should add 2 more lines.
            let growth = exponential.saturating_sub(1);
            let mut width = self.universe.width;
            let mut height = self.universe.height;
            let mut galaxies = self.universe.galaxies.clone();

            self.rows.iter().for_each(|row| {
                height += growth;
                galaxies
                    .iter_mut()
                    .filter(
                        |Galaxy {
                             position: Position { x: _, y },
                             ..
                         }| y > row,
                    )
                    .for_each(
                        |Galaxy {
                             shift: Position { x: _, y },
                             ..
                         }| *y += growth,
                    );
            });
            self.columns.iter().for_each(|column| {
                width += growth;
                galaxies
                    .iter_mut()
                    .filter(
                        |Galaxy {
                             position: Position { x, y: _ },
                             ..
                         }| x > column,
                    )
                    .for_each(
                        |Galaxy {
                             shift: Position { x, y: _ },
                             ..
                         }| *x += growth,
                    );
            });

            galaxies.iter_mut().for_each(|galaxy| galaxy.resolve());

            Universe {
                width,
                height,
                galaxies,
            }
        }
    }

    pub(crate) struct Universe {
        pub(crate) width: usize,
        pub(crate) height: usize,
        pub(crate) galaxies: Vec<Galaxy>,
    }
    impl TryFrom<&str> for Universe {
        type Error = Error;

        fn try_from(input: &str) -> Result<Self, Self::Error> {
            let input = input.trim();
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
                        let position = Position { x: x + 1, y: y + 1 };
                        if c == '#' {
                            galaxy_count += 1;
                            galaxies.push(Galaxy {
                                id: galaxy_count,
                                position,
                                shift: Position::default(),
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
    impl Universe {
        pub(crate) fn measure(&self) -> Spacing {
            Spacing {
                universe: self,
                rows: (1..=self.height)
                    .filter(|line| {
                        self.galaxies
                            .iter()
                            .filter(
                                |Galaxy {
                                     position: Position { x: _, y },
                                     ..
                                 }| y == line,
                            )
                            .count()
                            == 0
                    })
                    .collect(),
                columns: (1..=self.width)
                    .filter(|column| {
                        self.galaxies
                            .iter()
                            .filter(
                                |Galaxy {
                                     position: Position { x, y: _ },
                                     ..
                                 }| x == column,
                            )
                            .count()
                            == 0
                    })
                    .collect(),
            }
        }

        pub(crate) fn distances(&self) -> BTreeMap<(usize, usize), usize> {
            self.galaxies
                .iter()
                .combinations(2)
                .map(|pairs| {
                    let (first, second) = pairs.iter().collect_tuple().unwrap();
                    (
                        (first.id, second.id),
                        first.position.distance_from(second.position),
                    )
                })
                .collect()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use rstest::rstest;

        const TEST_UNIVERSE: &str = "
...#......
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
            assert_eq!(TEST_UNIVERSE.trim(), format!("{universe}").trim());
        }

        #[test]
        fn test_expansion() {
            let universe: Universe = TEST_UNIVERSE.try_into().unwrap();
            let spacing = universe.measure();
            assert_eq!(vec![4, 8], spacing.rows);
            assert_eq!(vec![3, 6, 9], spacing.columns);
            let expected_spacing: &str = "
   v  v  v
 ...#......
 .......#..
 #.........
>..........
 ......#...
 .#........
 .........#
>..........
 .......#..
 #...#.....
";
            assert_eq!(expected_spacing.trim(), format!("{spacing}").trim());
        }

        #[test]
        fn test_expanded_universe() {
            let universe: Universe = TEST_UNIVERSE.try_into().unwrap();
            let expanded = universe.measure().expand(2);
            let expected_universe: &str = "
....#........
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
#....#.......
";
            assert_eq!(expected_universe.trim(), format!("{expanded}").trim());
        }

        #[test]
        fn test_distances_calculated() {
            let universe: Universe = TEST_UNIVERSE.try_into().unwrap();
            let expanded = universe.measure().expand(2);
            let distances = expanded.distances();

            let num_galaxies = expanded.galaxies.len();
            let expected_num_pairs = (num_galaxies * (num_galaxies - 1)) / 2;
            assert_eq!(expected_num_pairs, distances.len());
        }

        #[test]
        fn test_distance() {
            let first = Position { x: 5, y: 9 };
            let second = Position { x: 7, y: 4 };
            assert_eq!(7, first.distance_from(second));
        }

        #[rstest]
        #[case((5, 9), 9)]
        #[case((1, 7), 15)]
        #[case((3, 6), 17)]
        #[case((8, 9), 5)]
        fn test_distance_between_galaxies(#[case] pair: (usize, usize), #[case] expected: usize) {
            let universe: Universe = TEST_UNIVERSE.try_into().unwrap();
            let expanded = universe.measure().expand(2);
            let distances = expanded.distances();
            assert_eq!(&expected, distances.get(&(pair.0, pair.1)).unwrap());
        }
    }
}
