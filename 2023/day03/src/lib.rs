use aoc_error::AocError;
use std::io::{Error, ErrorKind};

pub mod aoc_error;
mod display;
mod parser;
pub mod part1;
pub mod part2;

fn invalid_input_error(message: &str) -> AocError {
    AocError::IoError(Error::new(ErrorKind::InvalidInput, message))
}

// Structures for Initial Parsing of Input Text

type Engine<'a> = Vec<Line<'a>>;
type Line<'a> = Vec<Chunk<'a>>;
#[derive(Debug)]
enum Chunk<'a> {
    BlankSpace(&'a str),
    PartNumber(&'a str),
    Symbol(char),
}
impl<'a> Chunk<'a> {
    fn len(&self) -> usize {
        match self {
            Self::BlankSpace(s) => s.len(),
            Self::PartNumber(s) => s.len(),
            Self::Symbol(_) => 1,
        }
    }
}

// Structures for Mapping Positions of Engine Parts

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Symbol {
    symbol: char,
    coord: Coord,
}

#[derive(Debug)]
struct PartNumber {
    id: usize,
    length: usize,
    coord: Coord,
}
impl PartNumber {
    pub fn from_str(part: &str, x: usize, y: usize) -> Self {
        Self {
            id: part
                .parse()
                .expect("Part constructed with malformed ID number."),
            length: part.len(),
            coord: Coord { x, y },
        }
    }

    pub fn neighbours(&self, symbol: &Symbol) -> bool {
        let topleft = Coord {
            x: self.coord.x.saturating_sub(1),
            y: self.coord.y.saturating_sub(1),
        };
        let bottomright = Coord {
            x: self.coord.x.saturating_add(self.length),
            y: self.coord.y.saturating_add(1),
        };
        // Is the symbol within the bounding box?
        symbol.coord.x >= topleft.x
            && symbol.coord.x <= bottomright.x
            && symbol.coord.y >= topleft.y
            && symbol.coord.y <= bottomright.y
    }
}

type Parts = Vec<PartNumber>;
type Symbols = Vec<Symbol>;
#[derive(Debug)]
struct EngineMap {
    parts: Parts,
    symbols: Symbols,
}
impl From<Engine<'_>> for EngineMap {
    fn from(engine: Engine<'_>) -> Self {
        let mut parts: Vec<PartNumber> = vec![];
        let mut symbols: Vec<Symbol> = vec![];
        for (index, line) in engine.iter().enumerate() {
            // Starting from (1,1) makes more sense to me as we are indicating
            // the position of "boxes" rather than point coordinates.
            let y = index + 1;
            let mut x: usize = 1;
            for chunk in line {
                if let Chunk::PartNumber(number) = chunk {
                    parts.push(PartNumber::from_str(number, x, y));
                } else if let Chunk::Symbol(symbol) = chunk {
                    symbols.push(Symbol {
                        symbol: *symbol,
                        coord: Coord { x, y },
                    })
                }
                x += chunk.len();
            }
        }
        EngineMap { parts, symbols }
    }
}
type Gear<'a> = (&'a Symbol, Vec<&'a PartNumber>);
impl<'a> EngineMap {
    fn get_parts_neighbouring_any_symbol(&self) -> Vec<&PartNumber> {
        self.parts
            .iter()
            .filter(|part| -> bool { self.symbols.iter().any(|symbol| part.neighbours(symbol)) })
            .collect()
    }

    fn get_adjacent_parts(&self, symbol: &Symbol) -> Vec<&PartNumber> {
        self.parts
            .iter()
            .filter(|part| part.neighbours(symbol))
            .collect()
    }

    fn get_gears(&'a self) -> Vec<Gear<'a>> {
        self.symbols
            .iter()
            .filter(|symbol| symbol.symbol == '*')
            .map(|symbol| -> Gear<'a> { (symbol, self.get_adjacent_parts(symbol)) })
            .filter(|gear| gear.1.len() == 2)
            .collect()
    }
}
