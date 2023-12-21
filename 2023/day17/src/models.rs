use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

use crate::error::Error;

pub(crate) mod display;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Position {
    x: usize,
    y: usize,
}
impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn manhatten(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn is_out_of_bounds(&self, max: &Position) -> bool {
        self.x < 1 || self.x > max.x || self.y < 1 || self.y > max.y
    }
}
impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Position::new(value.0, value.1)
    }
}
impl From<&(usize, usize)> for Position {
    fn from(value: &(usize, usize)) -> Self {
        Position::new(value.0, value.1)
    }
}

#[derive(Eq, Clone, Debug)]
struct Block {
    position: Position,
    temperature_loss: usize,
}
impl Deref for Block {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.temperature_loss
    }
}
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}
impl Hash for Block {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.temperature_loss.hash(state);
    }
}

pub(crate) struct City {
    width: usize,
    height: usize,
    blocks: BTreeMap<Position, Block>,
}
impl TryFrom<&str> for City {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut blocks = BTreeMap::new();
        let height: usize = value.lines().count();
        let mut width: Option<usize> = None;
        for (y, line) in value.lines().enumerate() {
            let line_width = line.chars().count();
            if width.get_or_insert(line_width) != &line_width {
                return Err(Error::CouldNotDetermineCityWidth);
            }
            for (x, n) in line.chars().enumerate() {
                let temperature_loss: usize = n.to_digit(10).ok_or(Error::CouldNotParseBlockCost)? as usize;
                let position = Position { x: x + 1, y: y + 1 };
                blocks.insert(position, Block { position, temperature_loss });
            }
        }
        Ok(City {
            width: width.ok_or(Error::CouldNotDetermineCityWidth)?,
            height,
            blocks,
        })
    }
}
impl City {
    fn get_neighbouring_blocks(&self, position: &Position) -> Result<Vec<&Block>, Error> {
        let max = Position { x: self.width, y: self.height };
        if position.is_out_of_bounds(&max) {
            return Err(Error::InvalidCityBlock);
        }
        Ok([
            (position.x - 1, position.y),
            (position.x + 1, position.y),
            (position.x, position.y - 1),
            (position.x, position.y + 1),
        ]
        .into_iter()
        .map(Position::from)
        .filter(|position| !position.is_out_of_bounds(&max))
        .filter_map(|position| self.blocks.get(&position))
        .collect())
    }

    pub(crate) fn map_bottom_right(self) -> Map {
        let start = (1, 1).into();
        let finish = (self.width, self.height).into();
        match self.map(start, finish) {
            Ok(map) => map,
            Err(_) => unreachable!(),
        }
    }

    fn map(self, start: Position, finish: Position) -> Result<Map, Error> {
        Map::new(self, start, finish)
    }
}

#[derive(Eq, Clone, Debug)]
pub(crate) struct Edge {
    from: Position,
    to: Position,
    temperature_loss: usize,
    previous: Vec<Position>,
}
impl Edge {
    pub(crate) fn get_cost(&self) -> usize {
        self.temperature_loss
    }

    fn cant_go_forward(&self) -> Option<Position> {
        if self.previous.len() < 3 {
            return None;
        }

        // Grab the current, plus the previous 3 (4 positions in total).
        let mut check_path = vec![self.to];
        self.previous.iter().rev().take(3).for_each(|position| check_path.push(*position));

        let one_before = check_path.first().unwrap();
        let last = check_path.last().unwrap();

        // In the Y direction:
        if check_path.windows(2).all(|w| w[0].x == w[1].x) {
            Some(Position::new(
                one_before.x,
                match one_before.y.cmp(&last.y) {
                    Ordering::Greater => one_before.y + 1,
                    Ordering::Less => one_before.y - 1,
                    Ordering::Equal => unreachable!(),
                },
            ))
        } else if check_path.windows(2).all(|w| w[0].y == w[1].y) {
            Some(Position::new(
                match one_before.x.cmp(&last.x) {
                    Ordering::Greater => one_before.x + 1,
                    Ordering::Less => one_before.x - 1,
                    Ordering::Equal => unreachable!(),
                },
                one_before.y,
            ))
        } else {
            None
        }
    }
}
impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.from.eq(&other.from) && self.to.eq(&other.to)
    }
}
impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.from.hash(state);
        self.to.hash(state);
    }
}

pub(crate) struct Map {
    city: City,
    start: Position,
    finish: Position,
}
impl Map {
    pub(crate) fn new(city: City, start: Position, finish: Position) -> Result<Self, Error> {
        let max = (city.width, city.height).into();
        if start.is_out_of_bounds(&max) || finish.is_out_of_bounds(&max) {
            return Err(Error::InvalidCityBlock);
        }
        Ok(Self { city, start, finish })
    }

    fn start_edge(&self, start: Position) -> Result<Edge, Error> {
        if start.is_out_of_bounds(&(self.city.width, self.city.height).into()) {
            return Err(Error::InvalidCityBlock);
        }
        // Within the limits, should exist.
        let block = match self.city.blocks.get(&start) {
            Some(block) => block,
            None => unreachable!(),
        };
        Ok(Edge {
            from: block.position,
            to: block.position,
            temperature_loss: 0,
            previous: Default::default(),
        })
    }

    pub(crate) fn get_heuristic(&self, edge: &Edge) -> usize {
        edge.to.manhatten(&self.finish)
    }

    pub(crate) fn start(&self) -> Result<Edge, Error> {
        self.start_edge(self.start)
    }

    pub(crate) fn is_complete(&self, edge: &Edge) -> bool {
        edge.to == self.finish
    }

    pub(crate) fn get_edges(&self, edge: &Edge) -> Vec<Edge> {
        let banned_position = edge.cant_go_forward();
        // Hopefully the CPU's branch predictor will automatically optimize out
        // the conditional filter down below if this value is already in the stack.
        let should_additional_filter = banned_position.is_some();
        // For some really fucked up reason, the correct values get spit out
        // when I type-hint `Vec<&Block>` but the _incorrect_ values get spit
        // out if I don't type-hint `neighbouring_blocks` _even though it still
        // compiles fine_.
        let neighbouring_blocks: Vec<&Block> = self.city
            .get_neighbouring_blocks(&edge.to)
            // If there was an error fetching neighbouring blocks it means we're
            // out of bounds of the city. Return empty vector.
            .unwrap_or_else(|_| Vec::new());
        neighbouring_blocks
            .into_iter()
            // Filter out any blocks that have already been traversed.
            .filter(|block| edge.previous.iter().all(|position| position != &block.position))
            // If the crucible has been travelling for three straight blocks in
            // a row, disallow the next block in front.
            .filter(|block| !should_additional_filter || block.position != banned_position.unwrap())
            .filter_map(|block| self.get_edge(edge, &block.position).ok())
            .collect()
    }

    fn get_edge(&self, edge: &Edge, to: &Position) -> Result<Edge, Error> {
        if to.is_out_of_bounds(&(self.city.width, self.city.height).into()) {
            return Err(Error::InvalidCityBlock);
        }
        let block = match self.city.blocks.get(to) {
            Some(block) => block,
            None => unreachable!(),
        };
        // Because we're working in unit-length blocks, the potential differential
        // will always be ±1. Plus we're making the assumption that the input will
        // always have a temperature loss of greater than zero. Must be sure that we
        // never have negative travel costs (that is, the potential differential of
        // each edge must not be greater than the temperature loss of the "to" block.
        let mut previous = edge.previous.clone();
        previous.push(edge.to);
        Ok(Edge {
            from: edge.to,
            to: *to,
            temperature_loss: block.temperature_loss,
            previous,
        })
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    fn make_edge_path(path: &[(usize, usize)]) -> Edge {
        let mut edge: Option<Edge> = None;
        for w in path.windows(2) {
            edge = Some(make_edge(w[1].into(), w[1].into(), edge.clone()));
        }
        edge.expect("At least two positions must be passed")
    }

    fn make_edge(from: Position, to: Position, before: Option<Edge>) -> Edge {
        let mut previous: Vec<Position> = Vec::new();
        if let Some(before) = before {
            previous = before.previous.clone();
            previous.push(before.to);
        }
        Edge {
            from,
            to,
            temperature_loss: 1,
            previous,
        }
    }

    fn make_city(size: usize) -> City {
        let mut blocks = BTreeMap::new();
        for y in 1..=size {
            for x in 1..=size {
                let position = Position::new(x, y);
                blocks.insert(position, Block { position, temperature_loss: 1 });
            }
        }
        City { width: size, height: size, blocks }
    }

    #[rstest]
    #[case([(1, 2), (2, 2), (3, 2), (3, 3)], None)]
    #[case([(3, 3), (3, 4), (3, 5), (3, 6)], Some((3, 7)))]
    #[case([(3, 6), (3, 5), (3, 4), (3, 3)], Some((3, 2)))]
    #[case([(5, 5), (6, 5), (7, 5), (8, 5)], Some((9, 5)))]
    #[case([(7, 9), (6, 9), (5, 9), (4, 9)], Some((3, 9)))]
    fn test_straight_line(#[case] positions: [(usize, usize); 4], #[case] expected: Option<(usize, usize)>) {
        let edge = make_edge_path(&positions);
        if let Some(expected) = expected {
            let expected = Position::from(expected);
            assert_eq!(Some(expected), edge.cant_go_forward());
        } else {
            assert!(edge.cant_go_forward().is_none());
        }
    }

    #[rstest]
    #[case((4, 3), &[(4, 4), (4, 2), (5, 3), (3, 3)])]
    #[case((5, 5), &[(4, 5), (5, 4)])]
    #[case((4, 1), &[(3, 1), (5, 1), (4, 2)])]
    fn test_get_neighbouring_blocks(#[case] position: (usize, usize), #[case] expected: &[(usize, usize)]) {
        let city = make_city(5);
        let neighbouring_blocks: Vec<Position> =
            city.get_neighbouring_blocks(&position.into()).unwrap().iter().map(|block| block.position).collect();
        assert_eq!(expected.len(), neighbouring_blocks.len());
        expected.iter().map(Position::from).for_each(|position| {
            assert!(neighbouring_blocks.contains(&position));
        });
    }

    #[test]
    fn test_out_of_bounds() {
        let city = make_city(5);
        assert!(city.get_neighbouring_blocks(&(6, 6).into()).is_err());
    }

    #[rstest]
    #[case(&[(1, 1), (1, 2), (2, 2), (3, 2), (3, 3), (4, 3)], &[(4, 2), (5, 3), (4, 4)])]
    #[case(&[(1, 1), (2, 1), (3, 1), (4, 1)], &[(4, 2)])]
    #[case(&[(1, 1), (1, 2), (1, 3), (1, 4), (2, 4), (2, 3)], &[(2, 2), (3, 3)])]
    fn test_allowed_to_move_in_all_directions(
        #[case] path_taken: &[(usize, usize)],
        #[case] expected: &[(usize, usize)],
    ) {
        let edge = make_edge_path(path_taken);
        assert_eq!(edge.to, path_taken.last().map(Position::from).unwrap());
        let possible_positions_to_move_to = make_city(5)
            .map_bottom_right()
            .get_edges(&edge)
            .into_iter()
            .map(|edge| edge.to)
            .collect::<Vec<_>>();
        expected.iter().map(Position::from).for_each(|position| {
            assert!(possible_positions_to_move_to.contains(&position));
        });
        assert_eq!(expected.len(), possible_positions_to_move_to.len());
    }
}
