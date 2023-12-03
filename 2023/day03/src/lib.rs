use aoc_error::AocError;
use std::fmt;
use std::io::{Error, ErrorKind};

pub mod aoc_error;
mod parser;
pub mod part1;
pub mod part2;

fn invalid_input_error(message: &str) -> AocError {
    AocError::IoError(Error::new(ErrorKind::InvalidInput, message))
}

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

#[derive(Debug)]
struct Symbol {
    symbol: char,
    x: usize,
    y: usize,
}
impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}', ({},{})", self.symbol, self.x, self.y)
    }
}

#[derive(Debug)]
struct PartNumber {
    id: usize,
    length: usize,
    x: usize,
    y: usize,
}
impl PartNumber {
    pub fn new(part: &str, x: usize, y: usize) -> Self {
        Self {
            id: part
                .parse()
                .expect("Part constructed with malformed ID number."),
            length: part.len(),
            x,
            y,
        }
    }

    pub fn is_next_to_symbol(&self, symbol: &Symbol) -> bool {
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
impl fmt::Display for PartNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "#{} ({}-{},{})",
            self.id,
            self.x,
            self.x + self.length,
            self.y
        )
    }
}

type Parts = Vec<PartNumber>;
type Symbols = Vec<Symbol>;
#[derive(Debug)]
struct EngineMap {
    parts: Parts,
    symbols: Symbols,
}
impl fmt::Display for EngineMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut lines: Vec<String> = vec![];
        let max_width = ::std::cmp::max(
            self.parts.len().to_string().len(),
            self.symbols.len().to_string().len(),
        );

        lines.push("List of Machine Parts".to_string());
        for (index, part) in self.parts.iter().enumerate() {
            lines.push(format!("{:width$}: {}", index + 1, part, width = max_width));
        }

        lines.push("\nList of Symbols".to_string());
        for (index, symbol) in self.symbols.iter().enumerate() {
            lines.push(format!(
                "{:width$}: {}",
                index + 1,
                symbol,
                width = max_width
            ));
        }

        write!(f, "{}", lines.join("\n"))
    }
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
                    parts.push(PartNumber::new(number, x, y));
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
    fn get_parts_next_to_symbols(&self) -> Vec<&PartNumber> {
        self.parts
            .iter()
            .filter(|part| -> bool {
                self.symbols
                    .iter()
                    .filter(|symbol| part.is_next_to_symbol(symbol))
                    .count()
                    > 0
            })
            .collect()
    }
}
