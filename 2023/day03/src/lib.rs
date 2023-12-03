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
struct Symbol {
    symbol: char,
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct PartNumber {
    id: usize,
    length: usize,
    x: usize,
    y: usize,
}
impl PartNumber {
    pub fn from_str(part: &str, x: usize, y: usize) -> Self {
        Self {
            id: part
                .parse()
                .expect("Part constructed with malformed ID number."),
            length: part.len(),
            x,
            y,
        }
    }

    pub fn neighbours(&self, symbol: &Symbol) -> bool {
        let topleft = (self.x.saturating_sub(1), self.y.saturating_sub(1));
        let bottomright = (
            self.x.saturating_add(self.length).saturating_add(1),
            self.y.saturating_add(1),
        );
        // Is the symbol within the bounding box?
        symbol.x >= topleft.0
            && symbol.x <= bottomright.0
            && symbol.y >= topleft.1
            && symbol.y <= bottomright.1
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
            let y = index + 1;
            let mut x: usize = 1;
            for chunk in line {
                if let Chunk::PartNumber(number) = chunk {
                    parts.push(PartNumber::from_str(number, x, y));
                } else if let Chunk::Symbol(symbol) = chunk {
                    symbols.push(Symbol {
                        symbol: *symbol,
                        x,
                        y,
                    })
                }
                x += chunk.len();
            }
        }
        EngineMap { parts, symbols }
    }
}
impl EngineMap {
    fn get_parts_neighbouring_any_symbol(&self) -> Vec<&PartNumber> {
        self.parts
            .iter()
            .filter(|part| -> bool {
                self.symbols
                    .iter()
                    .find(|symbol| part.neighbours(symbol))
                    .is_some()
            })
            .collect()
    }
}
