use aoc_common::{ParseError, RunnerError, Solution};
use std::fmt::Display;

pub struct Day03 {}
impl Solution for Day03 {
    type Parsed = models::EngineMap;

    fn parse(&self, input: &str) -> Result<Self::Parsed, ParseError> {
        parser::parse(input)
    }

    fn part1(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        Ok(input.get_parts_neighbouring_any_symbol().iter().map(|part| part.get_id()).sum::<usize>())
    }

    fn part2(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        Ok(input
            .get_gears()
            .iter()
            .map(|gear| gear.1.iter().fold(1, |carry, part| carry * part.get_id()))
            .sum::<usize>())
    }
}

mod models {
    use std::cmp::{max, min};

    use aoc_common::ParseError;

    pub(crate) type Engine<'a> = Vec<Line<'a>>;
    pub(crate) type Line<'a> = Vec<Chunk<'a>>;
    #[derive(Debug)]
    pub(crate) enum Chunk<'a> {
        BlankSpace(&'a str),
        PartNumber(&'a str),
        Symbol(char),
    }
    #[derive(Clone, Debug, PartialEq)]
    pub(crate) struct Coord {
        x: usize,
        y: usize,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub(crate) struct Symbol {
        symbol: char,
        coord: Coord,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub(crate) struct PartNumber {
        id: usize,
        length: usize,
        coord: Coord,
    }
    type Parts = Vec<PartNumber>;
    type Symbols = Vec<Symbol>;
    #[derive(Clone, Debug, PartialEq)]
    pub struct EngineMap {
        parts: Parts,
        symbols: Symbols,
    }
    type Gear<'a> = (&'a Symbol, Vec<&'a PartNumber>);

    impl Chunk<'_> {
        pub(crate) fn len(&self) -> usize {
            match self {
                Self::BlankSpace(s) => s.len(),
                Self::PartNumber(s) => s.len(),
                Self::Symbol(_) => 1,
            }
        }
    }

    impl Coord {
        pub(crate) fn new(x: usize, y: usize) -> Self {
            Self { x, y }
        }

        fn is_bounded_by(&self, corner1: &Coord, corner2: &Coord) -> bool {
            self.x >= min(corner1.x, corner2.x)
                && self.x <= max(corner1.x, corner2.x)
                && self.y >= min(corner1.y, corner2.y)
                && self.y <= max(corner1.y, corner2.y)
        }
    }

    impl Symbol {
        pub(crate) fn new(symbol: char, coord: Coord) -> Self {
            Self { symbol, coord }
        }
    }

    impl PartNumber {
        pub(crate) fn get_id(&self) -> usize {
            self.id
        }

        pub(crate) fn from_str(part: &str, x: usize, y: usize) -> Result<Self, ParseError> {
            Ok(Self {
                id: part.parse().map_err(ParseError::Int)?,
                length: part.len(),
                coord: Coord { x, y },
            })
        }

        pub(crate) fn neighbours(&self, symbol: &Symbol) -> bool {
            let topleft = Coord {
                x: self.coord.x.saturating_sub(1),
                y: self.coord.y.saturating_sub(1),
            };
            let bottomright = Coord {
                x: self.coord.x.saturating_add(self.length),
                y: self.coord.y.saturating_add(1),
            };
            // Is the symbol within the bounding box?
            symbol.coord.is_bounded_by(&topleft, &bottomright)
        }
    }

    impl<'a> EngineMap {
        pub(crate) fn new(parts: Parts, symbols: Symbols) -> Self {
            Self { parts, symbols }
        }

        pub(crate) fn get_parts_neighbouring_any_symbol(&self) -> Vec<&PartNumber> {
            self.parts
                .iter()
                .filter(|part| -> bool { self.symbols.iter().any(|symbol| part.neighbours(symbol)) })
                .collect()
        }

        fn get_adjacent_parts(&self, symbol: &Symbol) -> Vec<&PartNumber> {
            self.parts.iter().filter(|part| part.neighbours(symbol)).collect()
        }

        pub(crate) fn get_gears(&'a self) -> Vec<Gear<'a>> {
            self.symbols
                .iter()
                .filter(|symbol| symbol.symbol == '*')
                .map(|symbol| -> Gear<'a> { (symbol, self.get_adjacent_parts(symbol)) })
                .filter(|gear| gear.1.len() == 2)
                .collect()
        }
    }
}

mod parser {
    use super::models::*;
    use aoc_common::ParseError;
    use nom::{
        branch::alt,
        character::complete::{char, digit1, line_ending, satisfy},
        combinator::{consumed, map},
        multi::many1,
        multi::separated_list1,
        IResult,
    };

    fn parse_line(input: &str) -> IResult<&str, Line> {
        let blankspace = map(consumed(many1(char('.'))), |(space_str, _spaces)| Chunk::BlankSpace(space_str));
        let partnumber = map(digit1, Chunk::PartNumber);
        let symbol = map(satisfy(|c| !c.is_ascii_digit() && c != '.' && c != '\n'), Chunk::Symbol);
        many1(alt((blankspace, partnumber, symbol)))(input)
    }

    fn parse_engine(input: &str) -> IResult<&str, Engine> {
        separated_list1(line_ending, parse_line)(input.trim())
    }

    pub(crate) fn parse(input: &str) -> Result<EngineMap, ParseError> {
        let (_, engine) = parse_engine(input).map_err(|e| ParseError::Nom(e.to_string()))?;
        let mut parts: Vec<PartNumber> = vec![];
        let mut symbols: Vec<Symbol> = vec![];
        for (index, line) in engine.iter().enumerate() {
            // Starting from (1,1) makes more sense to me as we are indicating
            // the position of "boxes" rather than point coordinates.
            let y = index + 1;
            let mut x: usize = 1;
            for chunk in line {
                if let Chunk::PartNumber(number) = chunk {
                    parts.push(PartNumber::from_str(number, x, y)?);
                } else if let Chunk::Symbol(symbol) = chunk {
                    symbols.push(Symbol::new(*symbol, Coord::new(x, y)))
                }
                x += chunk.len();
            }
        }
        Ok(EngineMap::new(parts, symbols))
    }
}

#[cfg(test)]
mod tests {
    use super::models::*;
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_parse() {
        let solution = Day03 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!(
            EngineMap::new(
                vec![
                    PartNumber::from_str("467", 1, 1).unwrap(),
                    PartNumber::from_str("114", 6, 1).unwrap(),
                    PartNumber::from_str("35", 3, 3).unwrap(),
                    PartNumber::from_str("633", 7, 3).unwrap(),
                    PartNumber::from_str("617", 1, 5).unwrap(),
                    PartNumber::from_str("58", 8, 6).unwrap(),
                    PartNumber::from_str("592", 3, 7).unwrap(),
                    PartNumber::from_str("755", 7, 8).unwrap(),
                    PartNumber::from_str("664", 2, 10).unwrap(),
                    PartNumber::from_str("598", 6, 10).unwrap(),
                ],
                vec![
                    Symbol::new('*', Coord::new(4, 2)),
                    Symbol::new('#', Coord::new(7, 4)),
                    Symbol::new('*', Coord::new(4, 5)),
                    Symbol::new('+', Coord::new(6, 6)),
                    Symbol::new('$', Coord::new(4, 9)),
                    Symbol::new('*', Coord::new(6, 9)),
                ]
            ),
            parsed
        );
    }

    #[test]
    fn test_part_one() {
        let solution = Day03 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!("4361", solution.part1(parsed).unwrap().to_string());
    }

    #[test]
    fn test_part_two() {
        let solution = Day03 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!("467835", solution.part2(parsed).unwrap().to_string());
    }
}
