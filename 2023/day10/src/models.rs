use crate::error::Error;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    fmt,
};

type Position = (usize, usize);

struct CanGoDirection {
    north: bool,
    south: bool,
    east: bool,
    west: bool,
}
impl From<Pipe> for CanGoDirection {
    fn from(pipe: Pipe) -> Self {
        match pipe {
            Pipe::Vertical => Self {
                north: true,
                east: false,
                south: true,
                west: false,
            },
            Pipe::Horizontal => Self {
                north: false,
                east: true,
                south: false,
                west: true,
            },
            Pipe::NorthEast => Self {
                north: true,
                east: true,
                south: false,
                west: false,
            },
            Pipe::NorthWest => Self {
                north: true,
                east: false,
                south: false,
                west: true,
            },
            Pipe::SouthEast => Self {
                north: false,
                east: true,
                south: true,
                west: false,
            },
            Pipe::SouthWest => Self {
                north: false,
                east: false,
                south: true,
                west: true,
            },
            Pipe::Start => Self {
                north: true,
                east: true,
                south: true,
                west: true,
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Start,
}
impl TryFrom<char> for Pipe {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            'S' => Self::Start,
            c => return Err(Error::InvalidPipeCharacter(c)),
        })
    }
}
impl fmt::Display for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let pipe = match self {
            Self::Vertical => "|",
            Self::Horizontal => "-",
            Self::NorthEast => "L",
            Self::NorthWest => "J",
            Self::SouthEast => "F",
            Self::SouthWest => "7",
            Self::Start => "S",
        };
        write!(f, "{pipe}")
    }
}

#[derive(PartialEq, Clone, Copy)]
pub(crate) struct Cell {
    pub(crate) position: Position,
    pub(crate) pipe: Pipe,
}

#[derive(Debug)]
pub(crate) struct Map {
    pub(crate) tiles: HashMap<Position, Cell>,
    pub(crate) start: Cell,
}
impl TryFrom<&str> for Map {
    type Error = Error;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut tiles = HashMap::new();
        let mut starting_position: Option<Cell> = None;
        input.trim().lines().enumerate().for_each(|(y, line)| {
            line.trim().chars().enumerate().for_each(|(x, c)| {
                if let Ok(pipe) = c.try_into() {
                    // We're counting "cells" not "points in space", start from (1, 1).
                    let position = (x + 1, y + 1);
                    if pipe == Pipe::Start {
                        starting_position = Some(Cell {
                            position,
                            pipe: Pipe::Start,
                        });
                    }
                    tiles.insert(position, Cell { position, pipe });
                }
            });
        });
        Ok(Self {
            tiles,
            start: starting_position.ok_or(Error::NoStartingPosition)?,
        })
    }
}
impl Map {
    pub(crate) fn branches(
        &self,
        Cell {
            position: (x, y),
            pipe,
        }: Cell,
    ) -> Vec<Cell> {
        let mut valid_moves = vec![];

        let can_go: CanGoDirection = pipe.into();
        // Going North
        if can_go.north {
            let north: Position = (x, y - 1);
            if let Some(Cell { pipe, .. }) = self.tiles.get(&north) {
                if pipe == &Pipe::Vertical
                    || pipe == &Pipe::SouthEast
                    || pipe == &Pipe::SouthWest
                    || pipe == &Pipe::Start
                {
                    valid_moves.push(Cell {
                        position: north,
                        pipe: *pipe,
                    });
                }
            }
        }
        // Going East
        if can_go.east {
            let east: Position = (x + 1, y);
            if let Some(Cell { pipe, .. }) = self.tiles.get(&east) {
                if pipe == &Pipe::Horizontal
                    || pipe == &Pipe::NorthWest
                    || pipe == &Pipe::SouthWest
                    || pipe == &Pipe::Start
                {
                    valid_moves.push(Cell {
                        position: east,
                        pipe: *pipe,
                    });
                }
            }
        }
        // Going South
        if can_go.south {
            let south: Position = (x, y + 1);
            if let Some(Cell { pipe, .. }) = self.tiles.get(&south) {
                if pipe == &Pipe::Vertical
                    || pipe == &Pipe::NorthEast
                    || pipe == &Pipe::NorthWest
                    || pipe == &Pipe::Start
                {
                    valid_moves.push(Cell {
                        position: south,
                        pipe: *pipe,
                    });
                }
            }
        }
        // Going West
        if can_go.west {
            let west: Position = (x - 1, y);
            if let Some(Cell { pipe, .. }) = self.tiles.get(&west) {
                if pipe == &Pipe::Horizontal
                    || pipe == &Pipe::NorthEast
                    || pipe == &Pipe::SouthEast
                    || pipe == &Pipe::Start
                {
                    valid_moves.push(Cell {
                        position: west,
                        pipe: *pipe,
                    });
                }
            }
        }
        valid_moves
    }

    pub(crate) fn circuit(&self) -> Result<Vec<Cell>, Error> {
        let branches = self.branches(self.start);
        match branches.len().cmp(&2) {
            // If there's less than two possible directions to travel then it
            // can't be a big loop.
            Ordering::Less => Err(Error::NoTraversalFound),
            // If there's exactly two possible directions to travel then just
            // pick one. No need to check everything again in the opposite
            // direction.
            Ordering::Equal => self
                .traverse(vec![self.start], branches[0])
                .ok_or(Error::NoTraversalFound),
            // Multiple branches, we'll have to test them all to figure out
            // which are valid.
            Ordering::Greater => self
                .traverse(vec![], self.start)
                .ok_or(Error::NoTraversalFound),
        }
    }

    fn traverse(&self, mut traversed: Vec<Cell>, cell: Cell) -> Option<Vec<Cell>> {
        traversed.push(cell);
        for branch in self.branches(cell) {
            // A circuit cannot be less than 5 cells (from start to start "S121S").
            // Check that it's at least 4 before adding the start position at
            // end and returning.
            if branch.pipe == Pipe::Start && traversed.len() >= 4 {
                traversed.push(branch);
                return Some(traversed);
            }
            if traversed.contains(&branch) {
                // One big loop means that there won't be any intersection of
                // paths, don't go backwards and don't get stuck in an infinite
                // loop!
                continue;
            }
            if let Some(path) = self.traverse(traversed.clone(), branch) {
                // Return early as soon as we find a branch that loops back to
                // the start.
                return Some(path);
            }
        }
        None
    }
}
