use aoc_error::AocError;
use parser::parse;
use serde::Serialize;
use std::{
    cmp::{max, min},
    ffi::{c_char, CStr, CString},
    io::{Error, ErrorKind},
};

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

// Exporting Functions for use in FFI

/// # Safety
/// This is unsafe. Because C is unsafe.
#[no_mangle]
pub unsafe extern "C" fn parse_engine_to_json(input: *const c_char) -> *const c_char {
    let input = unsafe { CStr::from_ptr(input) }.to_str().unwrap();
    let engine: EngineMap = parse(input).expect("Engine input could not be parsed.").into();
    let json = serde_json::to_string(&engine).unwrap();
    CString::new(json).unwrap().into_raw()
}

// Structures for Mapping Positions of Engine Parts

#[derive(Debug, Serialize)]
struct Coord {
    x: usize,
    y: usize,
}
impl Coord {
    fn is_bounded_by(&self, corner1: &Coord, corner2: &Coord) -> bool {
        self.x >= min(corner1.x, corner2.x)
            && self.x <= max(corner1.x, corner2.x)
            && self.y >= min(corner1.y, corner2.y)
            && self.y <= max(corner1.y, corner2.y)
    }
}

#[derive(Debug, Serialize)]
struct Symbol {
    symbol: char,
    coord: Coord,
}

#[derive(Debug, Serialize)]
struct PartNumber {
    id: usize,
    length: usize,
    coord: Coord,
}
impl PartNumber {
    pub fn from_str(part: &str, x: usize, y: usize) -> Self {
        Self {
            id: part.parse().expect("Part constructed with malformed ID number."),
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
        symbol.coord.is_bounded_by(&topleft, &bottomright)
    }
}

type Parts = Vec<PartNumber>;
type Symbols = Vec<Symbol>;
#[derive(Debug, Serialize)]
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
                    symbols.push(Symbol { symbol: *symbol, coord: Coord { x, y } })
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
        self.parts.iter().filter(|part| part.neighbours(symbol)).collect()
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
