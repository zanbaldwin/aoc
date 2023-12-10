use crate::error::Error;
use std::{collections::HashMap, fmt};

type Position = (usize, usize);

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum Direction {
    North,
    East,
    South,
    West,
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
impl Pipe {
    pub(crate) fn can_go(&self) -> Vec<Direction> {
        match self {
            Self::Vertical => vec![Direction::North, Direction::South],
            Self::Horizontal => vec![Direction::East, Direction::West],
            Self::NorthEast => vec![Direction::North, Direction::East],
            Self::NorthWest => vec![Direction::North, Direction::West],
            Self::SouthEast => vec![Direction::East, Direction::South],
            Self::SouthWest => vec![Direction::South, Direction::West],
            Self::Start => vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ],
        }
    }
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
    height: usize,
    width: usize,
    pub(crate) tiles: HashMap<Position, Cell>,
    pub(crate) start: Cell,
}
impl TryFrom<&str> for Map {
    type Error = Error;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut tiles = HashMap::new();
        let mut starting_position: Option<Cell> = None;
        let mut max_position: Option<Position> = None;
        input.trim().lines().enumerate().for_each(|(y, line)| {
            line.trim().chars().enumerate().for_each(|(x, c)| {
                // We're counting "cells" not "points in space", start from (1, 1).
                let position = (x + 1, y + 1);
                max_position = Some(position);
                if let Ok(pipe) = c.try_into() {
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
        let max_position = max_position.ok_or::<Error>("Maximum Boundaries not found.".into())?;
        Ok(Self {
            width: max_position.0,
            height: max_position.1,
            tiles,
            start: starting_position.ok_or(Error::NoStartingPosition)?,
        })
    }
}
impl Map {
    pub fn branches(
        &self,
        Cell {
            position: (x, y),
            pipe,
        }: Cell,
    ) -> Vec<Cell> {
        let mut branches = vec![];
        let can_go = pipe.can_go();
        // Going North
        if can_go.contains(&Direction::North) {
            let north: Position = (x, y - 1);
            if let Some(Cell { pipe, .. }) = self.tiles.get(&north) {
                if pipe == &Pipe::Vertical
                    || pipe == &Pipe::SouthEast
                    || pipe == &Pipe::SouthWest
                    || pipe == &Pipe::Start
                {
                    branches.push(Cell {
                        position: north,
                        pipe: *pipe,
                    });
                }
            }
        }
        // Going East
        if can_go.contains(&Direction::East) {
            let east: Position = (x + 1, y);
            if let Some(Cell { pipe, .. }) = self.tiles.get(&east) {
                if pipe == &Pipe::Horizontal
                    || pipe == &Pipe::NorthWest
                    || pipe == &Pipe::SouthWest
                    || pipe == &Pipe::Start
                {
                    branches.push(Cell {
                        position: east,
                        pipe: *pipe,
                    });
                }
            }
        }
        // Going South
        if can_go.contains(&Direction::South) {
            let south: Position = (x, y + 1);
            if let Some(Cell { pipe, .. }) = self.tiles.get(&south) {
                if pipe == &Pipe::Vertical
                    || pipe == &Pipe::NorthEast
                    || pipe == &Pipe::NorthWest
                    || pipe == &Pipe::Start
                {
                    branches.push(Cell {
                        position: south,
                        pipe: *pipe,
                    });
                }
            }
        }
        // Going West
        if can_go.contains(&Direction::West) {
            let west: Position = (x - 1, y);
            if let Some(Cell { pipe, .. }) = self.tiles.get(&west) {
                if pipe == &Pipe::Horizontal
                    || pipe == &Pipe::NorthEast
                    || pipe == &Pipe::SouthEast
                    || pipe == &Pipe::Start
                {
                    branches.push(Cell {
                        position: west,
                        pipe: *pipe,
                    });
                }
            }
        }
        branches
    }

    pub(crate) fn circuit(&self) -> Result<Vec<Cell>, Error> {
        let branches = self.branches(self.start);
        if branches.len() < 2 {
            return Err(Error::NoTraversalFound);
        }
        self.traverse(vec![], self.start)
            .ok_or(Error::NoTraversalFound)
    }

    fn traverse(&self, mut traversed: Vec<Cell>, cell: Cell) -> Option<Vec<Cell>> {
        traversed.push(cell);
        for branch in self.branches(cell) {
            if branch.pipe == Pipe::Start && traversed.len() >= 4 {
                traversed.push(branch);
                return Some(traversed);
            }
            if traversed.contains(&branch) {
                continue;
            }
            if let Some(path) = self.traverse(traversed.clone(), branch) {
                return Some(path);
            }
        }
        None
    }
}

// Part 2
impl Map {
    pub(crate) fn num_bounded(&self) -> Result<usize, Error> {
        let circuit = self.circuit()?;

        let mut valid_loop_cells: HashMap<Position, Cell> = HashMap::new();
        for cell in circuit.iter() {
            valid_loop_cells.insert(cell.position, *cell);
        }

        let mut count = 0;

        for y in 1..=self.height {
            for x in 1..=self.width {
                let position: Position = (x, y);
                if valid_loop_cells.get(&position).is_none()
                    && Map::is_position_bounded_by(position, &circuit)
                {
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    pub(crate) fn is_position_bounded_by((pos_x, pos_y): Position, circuit: &[Cell]) -> bool {
        // If it crosses an odd number of pipe boundaries on its way to the edge
        // in both directions, then it must be bounded by the looping circuit???
        let is_left_odd = circuit
            .iter()
            .filter(|cell| cell.position.0 < pos_x && cell.position.1 == pos_y)
            .count()
            % 2
            > 0;
        let is_right_odd = circuit
            .iter()
            .filter(|cell| cell.position.0 > pos_x && cell.position.1 == pos_y)
            .count()
            % 2
            > 0;
        let is_above_odd = circuit
            .iter()
            .filter(|cell| cell.position.0 == pos_x && cell.position.1 < pos_y)
            .count()
            % 2
            > 0;
        let is_below_odd = circuit
            .iter()
            .filter(|cell| cell.position.0 == pos_x && cell.position.1 > pos_y)
            .count()
            % 2
            > 0;
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MAP_1: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const TEST_MAP_2: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

    #[test]
    fn num_bounded() {
        let map: Map = TEST_MAP_1.try_into().unwrap();
        assert_eq!(4, map.num_bounded().unwrap());
    }

    #[test]
    fn num_bounded_no_gap() {
        let map: Map = TEST_MAP_2.try_into().unwrap();
        assert_eq!(4, map.num_bounded().unwrap());
    }

    #[test]
    fn is_position_bounded_by1() {
        let map: Map = TEST_MAP_1.try_into().unwrap();
        let circuit = map.circuit().unwrap();
        assert!(Map::is_position_bounded_by((3, 7), &circuit));
    }

    #[test]
    fn is_position_bounded_by2() {
        let map: Map = TEST_MAP_1.try_into().unwrap();
        let circuit = map.circuit().unwrap();
        assert!(!Map::is_position_bounded_by((1, 1), &circuit));
    }

    #[test]
    fn is_position_bounded_by3() {
        let map: Map = TEST_MAP_1.try_into().unwrap();
        let circuit = map.circuit().unwrap();
        assert!(!Map::is_position_bounded_by((4, 3), &circuit));
    }
}
