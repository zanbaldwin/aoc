use std::collections::btree_map::{IntoIter, Iter, IterMut};
use std::{collections::BTreeMap, fmt::Display};

/// One-based positioning
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    x: usize,
    y: usize,
}
impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    /// Create from zero-based indexing (eg, enumerates).
    pub fn from_index(x: usize, y: usize) -> Self {
        Self::new(x + 1, y + 1)
    }
    fn adjacent(&self) -> Vec<Position> {
        [
            Position::new(self.x.saturating_sub(1), self.y.saturating_sub(1)),
            Position::new(self.x, self.y.saturating_sub(1)),
            Position::new(self.x.saturating_add(1), self.y.saturating_sub(1)),
            Position::new(self.x.saturating_sub(1), self.y),
            Position::new(self.x.saturating_add(1), self.y),
            Position::new(self.x.saturating_sub(1), self.y.saturating_add(1)),
            Position::new(self.x, self.y.saturating_add(1)),
            Position::new(self.x.saturating_add(1), self.y.saturating_add(1)),
        ]
        .into_iter()
        .filter(|p| p.x > 0 && p.y > 0)
        .collect()
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Grid<T> {
    elements: BTreeMap<Position, T>,
}
impl<T> IntoIterator for Grid<T> {
    type Item = (Position, T);
    type IntoIter = IntoIter<Position, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}
impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = (&'a Position, &'a T);
    type IntoIter = Iter<'a, Position, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}
impl<'a, T> IntoIterator for &'a mut Grid<T> {
    type Item = (&'a Position, &'a mut T);
    type IntoIter = IterMut<'a, Position, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter_mut()
    }
}
impl<T> Grid<T> {
    pub fn iter(&self) -> Iter<'_, Position, T> {
        self.into_iter()
    }

    pub fn from_text<E, F>(input: &str, map: F) -> Result<Self, E>
    where
        F: Fn(char) -> Result<T, E>,
    {
        let elements: Result<BTreeMap<Position, T>, E> = input
            .trim_start()
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, symbol)| {
                        let position = Position::from_index(x, y);
                        map(symbol).map(|result| (position, result))
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        elements.map(Self::new)
    }

    pub fn new(elements: BTreeMap<Position, T>) -> Self {
        Self { elements }
    }

    pub fn exists(&self, position: &Position) -> bool {
        self.elements.contains_key(position)
    }

    pub fn get(&self, position: &Position) -> Option<&T> {
        self.elements.get(position)
    }

    pub fn get_mut(&mut self, position: &Position) -> Option<&mut T> {
        self.elements.get_mut(position)
    }

    pub fn get_adjacent(&self, position: &Position) -> Vec<(Position, &T)> {
        position.adjacent().into_iter().filter_map(|p| self.get(&p).map(move |e| (p, e))).collect()
    }

    pub fn remove(&mut self, position: &Position) -> Option<T> {
        self.elements.remove(position)
    }
}

impl Grid<char> {
    pub fn from_chars(input: &str) -> Self {
        Self::from_text(input, Ok::<char, ()>).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = r#"
abc
def
ghi
"#;

    fn construct() -> Grid<char> {
        Grid {
            elements: BTreeMap::from([
                (Position { x: 1, y: 1 }, 'a'),
                (Position { x: 2, y: 1 }, 'b'),
                (Position { x: 3, y: 1 }, 'c'),
                (Position { x: 1, y: 2 }, 'd'),
                (Position { x: 2, y: 2 }, 'e'),
                (Position { x: 3, y: 2 }, 'f'),
                (Position { x: 1, y: 3 }, 'g'),
                (Position { x: 2, y: 3 }, 'h'),
                (Position { x: 3, y: 3 }, 'i'),
            ]),
        }
    }

    #[test]
    fn test_from_input() {
        let grid: Grid<char> = Grid::from_chars(TEST_INPUT);
        assert_eq!(grid, construct());
    }
}
