use crate::{error::Error, models::*};
use std::fmt::{Debug, Display};

impl TryFrom<&str> for Cell {
    type Error = Error;
    fn try_from(cell: &str) -> Result<Self, Self::Error> {
        // Example: S(1,1)
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

// This reports as dead code because all the tests are removed from the analyzer during a check.
#[allow(dead_code)]
pub(crate) fn cell(input: &str) -> Cell {
    input
        .try_into()
        .expect("expect you to type cell test values correctly")
}
