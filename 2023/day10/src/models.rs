use crate::error::Error;
use std::collections::{BTreeMap, HashMap};

pub(crate) type Position = (usize, usize);

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
            Self::Start => vec![Direction::North, Direction::East, Direction::South, Direction::West],
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

#[derive(PartialEq, Clone, Copy)]
pub(crate) struct Cell {
    pub(crate) position: Position,
    pub(crate) pipe: Pipe,
}

#[derive(Debug)]
pub(crate) struct Map {
    pub(crate) height: usize,
    pub(crate) width: usize,
    pub(crate) tiles: HashMap<Position, Cell>,
    pub(crate) start: Cell,
}
impl TryFrom<&str> for Map {
    type Error = Error;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut tiles = HashMap::new();
        let mut starting_position: Option<Cell> = None;
        let height = input.lines().count();
        let mut width: Option<usize> = None;
        input.trim().lines().enumerate().for_each(|(y, line)| {
            width.get_or_insert_with(|| line.chars().count());
            line.trim().chars().enumerate().for_each(|(x, c)| {
                // We're counting "cells" not "points in space", start from (1, 1).
                let position = (x + 1, y + 1);
                if let Ok(pipe) = c.try_into() {
                    if pipe == Pipe::Start {
                        starting_position = Some(Cell { position, pipe: Pipe::Start });
                    }
                    tiles.insert(position, Cell { position, pipe });
                }
            });
        });
        Ok(Self {
            width: width.ok_or::<Error>("Could not determine grid width.".into())?,
            height,
            tiles,
            start: starting_position.ok_or(Error::NoStartingPosition)?,
        })
    }
}
impl Map {
    pub fn branches(&self, Cell { position: (x, y), pipe }: Cell) -> Vec<Cell> {
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
                    branches.push(Cell { position: north, pipe: *pipe });
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
                    branches.push(Cell { position: east, pipe: *pipe });
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
                    branches.push(Cell { position: south, pipe: *pipe });
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
                    branches.push(Cell { position: west, pipe: *pipe });
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
        self.traverse(vec![], self.start).ok_or(Error::NoTraversalFound)
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
    fn circuit_into_btree(circuit: Vec<Cell>) -> BTreeMap<Position, Cell> {
        circuit.into_iter().map(|cell| (cell.position, cell)).collect()
    }

    pub(crate) fn num_bounded(&self) -> Result<usize, Error> {
        let circuit = Map::circuit_into_btree(self.circuit()?);
        let mut count = 0;
        for y in 1..=self.height {
            for x in 1..=self.width {
                let position: Position = (x, y);
                if circuit.get(&position).is_none() && Map::is_position_bounded_by(position, &circuit) {
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    /// This is the part of my solution that I needed to look
    pub(crate) fn is_position_bounded_by((pos_x, pos_y): Position, circuit: &BTreeMap<Position, Cell>) -> bool {
        let bounding_pipes_to_the_left =
            circuit.iter().filter(|((cell_x, cell_y), _)| cell_x < &pos_x && cell_y == &pos_y);
        let mut crossings = 0;
        let mut previous: Option<Pipe> = None;
        for (_position, cell) in bounding_pipes_to_the_left {
            let pipe = cell.pipe;
            match pipe {
                // Crossing a vertical pipe always results in a crossing.
                Pipe::Vertical => crossings += 1,
                // Because we're coming in from the West, all pipes pointing East count as a crossing.
                Pipe::NorthEast => {
                    crossings += 1;
                    previous = Some(Pipe::NorthEast);
                },
                Pipe::SouthEast => {
                    crossings += 1;
                    previous = Some(Pipe::SouthEast);
                },
                // But pipes pointing east only count as another crossing if the
                // pipe is pointing the same north or south as the previous one.
                // For example:
                //   One crossing = ┗━┓ or ┏━┛
                //   Two crossings = ┏━┓ or ┗━┛
                Pipe::NorthWest if previous == Some(Pipe::NorthEast) => crossings += 1,
                Pipe::SouthWest if previous == Some(Pipe::SouthEast) => crossings += 1,
                _ => (),
            }
        }

        // Need an odd number of crossings to be inside the bounding box.
        crossings % 2 > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // ...........
    // .S━━━━━━━┓.
    // .┃┏━━━━━┓┃.
    // .┃┃.....┃┃.
    // .┃┃.....┃┃.
    // .┃┗━┓.┏━┛┃.
    // .┃..┃.┃..┃.
    // .┗━━┛.┗━━┛.
    // ...........
    const TEST_MAP_1: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    // ..........
    // .S━━━━━━┓.
    // .┃┏━━━━┓┃.
    // .┃┃....┃┃.
    // .┃┃....┃┃.
    // .┃┗━┓┏━┛┃.
    // .┃..┃┃..┃.
    // .┗━━┛┗━━┛.
    // ..........
    const TEST_MAP_2: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

    // .┏━━━━┓┏┓┏┓┏┓┏━┓....
    // .┃┏━━┓┃┃┃┃┃┃┃┃┏┛....
    // .┃┃.┏┛┃┃┃┃┃┃┃┃┗┓....
    // ┏┛┗┓┗┓┗┛┗┛┃┃┗┛.┗━┓..
    // ┗━━┛.┗┓...┗┛S┓┏━┓┗┓.
    // ....┏━┛..┏┓┏┛┃┗┓┗┓┗┓
    // ....┗┓.┏┓┃┃┗┓┃.┗┓┗┓┃
    // .....┃┏┛┗┛┃┏┛┃┏┓┃.┗┛
    // ....┏┛┗━┓.┃┃.┃┃┃┃...
    // ....┗━━━┛.┗┛.┗┛┗┛...
    const TEST_MAP_3: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    // ┌┏┓┏S┏┓┏┓┏┓┏┓┏┓┏━━━┓
    // └┃┗┛┃┃┃┃┃┃┃┃┃┃┃┃┏━━┛
    // ┌┗━┓┗┛┗┛┃┃┃┃┃┃┗┛┗━┓┐
    // ┏━━┛┏━━┓┃┃┗┛┗┛┐┏┓┏┛─
    // ┗━━━┛┏━┛┗┛.││─┏┛┗┛┘┐
    // │┌│┏━┛┏━━━┓┌┐─┗┓└│┐│
    // │┌┏┛┏┓┗┓┏━┛┏┓│┘┗━━━┓
    // ┐─┗━┛┗┓┃┃┏┓┃┗┓┏━┓┏┓┃
    // └.└┐└┏┛┃┃┃┃┃┏┛┗┓┃┃┗┛
    // └┐┘└┘┗━┛┗┛┗┛┗━━┛┗┛.└
    const TEST_MAP_4: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[rstest]
    #[case(TEST_MAP_1, 4)]
    #[case(TEST_MAP_2, 4)]
    #[case(TEST_MAP_3, 8)]
    #[case(TEST_MAP_4, 10)]
    fn num_bounded(#[case] input: &str, #[case] expected: usize) {
        let map: Map = input.try_into().unwrap();
        assert_eq!(expected, map.num_bounded().unwrap());
    }

    #[rstest]
    #[case(TEST_MAP_1, (3, 7), true)]
    #[case(TEST_MAP_1, (1, 1), false)]
    #[case(TEST_MAP_1, (4, 4), false)]
    #[case(TEST_MAP_3, (7, 7), true)]
    #[case(TEST_MAP_3, (8, 5), true)]
    #[case(TEST_MAP_4, (13, 6), true)]
    #[case(TEST_MAP_4, (19, 5), false)]
    fn is_position_bounded_by(#[case] input: &str, #[case] position: Position, #[case] expected: bool) {
        let map: Map = input.try_into().unwrap();
        let circuit = Map::circuit_into_btree(map.circuit().unwrap());
        assert_eq!(expected, Map::is_position_bounded_by(position, &circuit));
    }
}
