use crate::{error::Error, models::*};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

impl TryFrom<&str> for Cell {
    type Error = Error;
    fn try_from(cell: &str) -> Result<Self, Self::Error> {
        let length = cell.len();
        if length < 6 {
            return Err(Error::CellParseError);
        }
        let pipe: Pipe = cell
            .chars()
            .next()
            .ok_or(Error::CellParseError)?
            .try_into()?;
        let (x, y) = cell[2..length - 1]
            .split_once(',')
            .ok_or(Error::CellParseError)?;
        Ok(Cell {
            position: (
                x.parse().map_err(|_| Error::CellParseError)?,
                y.parse().map_err(|_| Error::CellParseError)?,
            ),
            pipe,
        })
    }
}
impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cell {:?}: [{:?}]", self.position, self.pipe)
    }
}
impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pipe = match self {
            Self::Vertical => "┃",
            Self::Horizontal => "━",
            Self::NorthEast => "┗",
            Self::NorthWest => "┛",
            Self::SouthEast => "┏",
            Self::SouthWest => "┓",
            Self::Start => "S",
        };
        write!(f, "{pipe}")
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let circuit_hashmap: HashMap<Position, Cell> = self
            .circuit()
            .map_err(|_| std::fmt::Error {})?
            .into_iter()
            .map(|cell| (cell.position, cell))
            .collect();
        let mut v: Vec<String> = vec![];
        for y in 1..=self.height {
            let mut line: String = String::new();
            for x in 1..=self.width {
                let position = (x, y);
                if let Some(tile) = self.tiles.get(&position) {
                    if circuit_hashmap.get(&position).is_some() {
                        line = format!("{line}{}", tile.pipe);
                    } else {
                        line = format!(
                            "{line}{}",
                            match tile.pipe {
                                Pipe::Vertical => '│',
                                Pipe::Horizontal => '─',
                                Pipe::NorthEast => '└',
                                Pipe::NorthWest => '┘',
                                Pipe::SouthEast => '┌',
                                Pipe::SouthWest => '┐',
                                Pipe::Start => 'S',
                            }
                        );
                    }
                } else {
                    line = format!("{line}.");
                }
            }
            v.push(line);
        }
        write!(f, "({}x{})\n{}", self.width, self.height, v.join("\n"))
    }
}

// This reports as dead code because all the tests are removed from the analyzer during a check.
#[allow(dead_code)]
pub(crate) fn cell(input: &str) -> Cell {
    input
        .try_into()
        .expect("expect you to type cell test values correctly")
}
