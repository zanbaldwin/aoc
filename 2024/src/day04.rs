use aoc_common::{ParseError, RunnerError, Solution};
use models::{Direction, Words, XmasDiscovery};
use std::{collections::BTreeMap, fmt::Display};

pub struct Day04 {}
impl Solution for Day04 {
    type Parsed = models::WordSearch;

    fn parse(&self, input: &str) -> Result<Self::Parsed, ParseError> {
        let lines = input.trim().lines();
        let mut height = 0;
        let mut width: Option<usize> = None;
        let mut characters = BTreeMap::new();
        lines.enumerate().for_each(|(y, line)| {
            height = y;
            let mut line_width = 0;

            line.trim().chars().enumerate().for_each(|(x, c)| {
                line_width = x;
                characters.insert(models::Position::new(x + 1, y + 1), c);
            });
            line_width += 1;
            if &line_width != width.get_or_insert(line_width) {
                // TODO: Bubble up this error instead of a panic.
                panic!("input contains variable width word search");
            }
        });
        Ok(models::WordSearch::new(
            characters,
            width.ok_or_else(|| ParseError::Custom("Undeterminable word search width".to_string()))?,
            height + 1,
        ))
    }

    fn part1(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        let x_char_positions =
            input.characters().iter().filter(|(_, c)| c == &&'X').map(|(position, _)| position).collect::<Vec<_>>();
        let mut words: Vec<XmasDiscovery> = Vec::new();
        for position in x_char_positions {
            let south_enough = position.y >= 4;
            let east_enough = position.x >= 4;
            let north_enough = position.y <= (input.height() - 4);
            let west_enough = position.x <= (input.width() - 4);

            if south_enough {
                let word = (
                    input.get_char(position.x, position.y).unwrap(),
                    input.get_char(position.x, position.y - 1).unwrap(),
                    input.get_char(position.x, position.y - 2).unwrap(),
                    input.get_char(position.x, position.y - 3).unwrap(),
                );
                let discovery = XmasDiscovery(position.clone(), Direction::North, word);
                if discovery.valid() {
                    words.push(discovery);
                }
            }
            if south_enough && west_enough {
                let word = (
                    input.get_char(position.x, position.y).unwrap(),
                    input.get_char(position.x + 1, position.y - 1).unwrap(),
                    input.get_char(position.x + 2, position.y - 2).unwrap(),
                    input.get_char(position.x + 3, position.y - 3).unwrap(),
                );
                let discovery = XmasDiscovery(position.clone(), Direction::NorthEast, word);
                if discovery.valid() {
                    words.push(discovery);
                }
            }
            if west_enough {
                let word = (
                    input.get_char(position.x, position.y).unwrap(),
                    input.get_char(position.x + 1, position.y).unwrap(),
                    input.get_char(position.x + 2, position.y).unwrap(),
                    input.get_char(position.x + 3, position.y).unwrap(),
                );
                let discovery = XmasDiscovery(position.clone(), Direction::East, word);
                if discovery.valid() {
                    words.push(discovery);
                }
            }
            if west_enough && north_enough {
                let word = (
                    input.get_char(position.x, position.y).unwrap(),
                    input.get_char(position.x + 1, position.y + 1).unwrap(),
                    input.get_char(position.x + 2, position.y + 2).unwrap(),
                    input.get_char(position.x + 3, position.y + 3).unwrap(),
                );
                let discovery = XmasDiscovery(position.clone(), Direction::SouthEast, word);
                if discovery.valid() {
                    words.push(discovery);
                }
            }
            if north_enough {
                let word = (
                    input.get_char(position.x, position.y).unwrap(),
                    input.get_char(position.x, position.y + 1).unwrap(),
                    input.get_char(position.x, position.y + 2).unwrap(),
                    input.get_char(position.x, position.y + 3).unwrap(),
                );
                let discovery = XmasDiscovery(position.clone(), Direction::South, word);
                if discovery.valid() {
                    words.push(discovery);
                }
            }
            if north_enough && east_enough {
                let word = (
                    input.get_char(position.x, position.y).unwrap(),
                    input.get_char(position.x - 1, position.y + 1).unwrap(),
                    input.get_char(position.x - 2, position.y + 2).unwrap(),
                    input.get_char(position.x - 3, position.y + 3).unwrap(),
                );
                let discovery = XmasDiscovery(position.clone(), Direction::SouthWest, word);
                if discovery.valid() {
                    words.push(discovery);
                }
            }
            if east_enough {
                let word = (
                    input.get_char(position.x, position.y).unwrap(),
                    input.get_char(position.x - 1, position.y).unwrap(),
                    input.get_char(position.x - 2, position.y).unwrap(),
                    input.get_char(position.x - 3, position.y).unwrap(),
                );
                let discovery = XmasDiscovery(position.clone(), Direction::West, word);
                if discovery.valid() {
                    words.push(discovery);
                }
            }
            if east_enough && south_enough {
                let word = (
                    input.get_char(position.x, position.y).unwrap(),
                    input.get_char(position.x - 1, position.y - 1).unwrap(),
                    input.get_char(position.x - 2, position.y - 2).unwrap(),
                    input.get_char(position.x - 3, position.y - 3).unwrap(),
                );
                let discovery = XmasDiscovery(position.clone(), Direction::NorthWest, word);
                if discovery.valid() {
                    words.push(discovery);
                }
            }
        }
        let output = Words { words };
        eprintln!("{output:?}");
        Ok(output)
    }

    fn part2(&self, _input: Self::Parsed) -> Result<impl Display, RunnerError> {
        Err::<usize, RunnerError>(RunnerError::Unimplemented)
    }
}

mod models {
    use std::{
        collections::BTreeMap,
        fmt::{Debug, Display},
    };

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub(crate) struct Position {
        pub(crate) x: usize,
        pub(crate) y: usize,
    }
    impl Position {
        pub(crate) fn new(x: usize, y: usize) -> Self {
            Self { x, y }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub(crate) enum Direction {
        North,
        NorthEast,
        East,
        SouthEast,
        South,
        SouthWest,
        West,
        NorthWest,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct WordSearch {
        characters: BTreeMap<Position, char>,
        width: usize,
        height: usize,
    }
    impl WordSearch {
        pub(crate) fn new(characters: BTreeMap<Position, char>, width: usize, height: usize) -> Self {
            Self { characters, width, height }
        }

        pub(crate) fn characters(&self) -> &BTreeMap<Position, char> {
            &self.characters
        }

        pub(crate) fn width(&self) -> usize {
            self.width
        }

        pub(crate) fn height(&self) -> usize {
            self.height
        }

        pub(crate) fn get_char(&self, x: usize, y: usize) -> Option<char> {
            self.characters.get(&Position { x, y }).copied()
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub(crate) struct XmasDiscovery(pub(crate) Position, pub(crate) Direction, pub(crate) (char, char, char, char));
    impl XmasDiscovery {
        pub(crate) fn valid(&self) -> bool {
            return self.2 == ('X', 'M', 'A', 'S');
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub(crate) struct Words {
        pub(crate) words: Vec<XmasDiscovery>,
    }
    impl Display for Words {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.words.len())
        }
    }
}

mod parser {}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    const PART1_SOLVED: &str = "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX";

    const PART2_SOLVED: &str = "";

    #[test]
    fn test_parse() {
        let solution = Day04 {};
        let parsed = solution.parse(INPUT).unwrap();
        // assert_eq!(Words { words: vec![] }, parsed);
    }

    #[test]
    fn test_part_one() {
        let solution = Day04 {};
        let parsed = solution.parse(INPUT).unwrap();
        eprintln!("{parsed:?}");
        let output = solution.part1(parsed).unwrap();
        assert_eq!("19", output.to_string());
    }

    #[test]
    fn test_part_two() {
        let solution = Day04 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!("", solution.part2(parsed).unwrap().to_string());
    }
}
