use crate::error::Error;
use std::collections::BTreeMap;

#[cfg(debug_assertions)]
mod display;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Position {
    pub(crate) x: usize,
    pub(crate) y: usize,
}
impl Position {
    pub(crate) fn is_out_of_bounds(&self, max_position: &Position) -> bool {
        self.x < 1 || self.x > max_position.x || self.y < 1 || self.y > max_position.y
    }
}

/// I have lost many hours across many AoC challenges simply because my brain
/// can't tell the difference between east and west. Just stick to left and
/// right for today.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn is_horizontal(&self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }

    fn is_vertical(&self) -> bool {
        matches!(self, Self::Up | Self::Down)
    }
}

#[derive(Clone)]
enum Splitter {
    Horizontal,
    Vertical,
}

#[derive(Clone)]
enum Mirror {
    ForwardSlash,
    BackSlash,
}

#[derive(Clone)]
enum Tile {
    Mirror(Mirror),
    Splitter(Splitter),
}

#[derive(Clone)]
struct Beam {
    position: Position,
    direction: Direction,
}
impl Beam {
    fn push(&mut self, direction: Option<Direction>) {
        let direction = direction.unwrap_or_else(|| self.direction.clone());
        match direction {
            Direction::Up => self.position.y -= 1,
            Direction::Down => self.position.y += 1,
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1,
        };
        self.direction = direction;
    }

    fn split(&self, direction: Direction) -> Self {
        let mut position = self.position;
        match direction {
            Direction::Up => position.y -= 1,
            Direction::Down => position.y += 1,
            Direction::Left => position.x -= 1,
            Direction::Right => position.x += 1,
        }
        Self { position, direction }
    }
}

#[derive(Clone)]
pub(crate) struct Contraption {
    width: usize,
    height: usize,
    tiles: BTreeMap<Position, Tile>,
    beams: Vec<Beam>,
    energized: BTreeMap<Position, Vec<Direction>>,
}
impl TryFrom<&str> for Contraption {
    type Error = Error;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut tiles = BTreeMap::new();
        let height: usize = input.lines().count();
        let mut width: Option<usize> = None;
        for (y, line) in input.lines().enumerate() {
            let line_width = line.chars().count();
            if width.get_or_insert(line_width) != &line_width {
                return Err(Error::CouldNotDetermineContraptionWidth);
            }
            for (x, c) in line.chars().enumerate() {
                let position = Position { x: x + 1, y: y + 1 };
                let tile = match c {
                    '/' => Tile::Mirror(Mirror::ForwardSlash),
                    '\\' => Tile::Mirror(Mirror::BackSlash),
                    '|' => Tile::Splitter(Splitter::Vertical),
                    '-' => Tile::Splitter(Splitter::Horizontal),
                    _ => continue,
                };
                tiles.insert(position, tile);
            }
        }
        Contraption::new(width.ok_or(Error::CouldNotDetermineContraptionWidth)?, height, tiles)
    }
}
impl Contraption {
    fn new(width: usize, height: usize, tiles: BTreeMap<Position, Tile>) -> Result<Self, Error> {
        let mut contraption = Self {
            width,
            height,
            tiles,
            beams: Default::default(),
            energized: Default::default(),
        };
        contraption.initialize(Position { x: 1, y: 1 }, Direction::Right)?;
        Ok(contraption)
    }

    /// Repeating Beam?
    ///
    /// Has a beam of light already passed through this tile in the same direction?
    fn is_beam_repeating(&self, beam: &Beam) -> bool {
        if let Some(directions) = self.energized.get(&beam.position) {
            directions.contains(&beam.direction)
        } else {
            false
        }
    }

    /// Max Iterations
    ///
    /// Get the maximum number of iterations that the contraption could possibly
    /// go through. That is, a beam of light energizing each tile from each
    /// possible direction. If the contraption has not completed after this many
    /// iterations, consider it in an infinite loop (which is a bug).
    pub(crate) fn get_max_iterations(&self) -> usize {
        (self.width * self.height * 4) + 1
    }

    pub(crate) fn initialize(&mut self, position: Position, direction: Direction) -> Result<(), Error> {
        if position.x > self.width
            && position.y > self.height
            && match direction {
                Direction::Up => position.y != self.height,
                Direction::Down => position.y != 1,
                Direction::Left => position.x != self.width,
                Direction::Right => position.x != 1,
            }
        {
            return Err(Error::InvalidStartingPositionOrDirection);
        }

        self.energized.clear();
        self.beams = vec![Beam { position, direction }];
        Ok(())
    }

    pub(crate) fn complete(&self) -> bool {
        self.beams.is_empty()
    }

    pub(crate) fn get_dimensions(&self) -> Position {
        Position { x: self.width, y: self.height }
    }

    pub(crate) fn step(&mut self) {
        use Direction::*;
        let dimensions = self.get_dimensions();
        let mut new_beams: Vec<Beam> = Vec::new();
        for beam in self.beams.iter_mut() {
            self.energized.entry(beam.position).or_default().push(beam.direction.clone());
            if let Some(tile) = self.tiles.get(&beam.position) {
                match tile {
                    Tile::Mirror(mirror) => match mirror {
                        Mirror::ForwardSlash => match beam.direction {
                            Up => beam.push(Some(Right)),
                            Down => beam.push(Some(Left)),
                            Left => beam.push(Some(Down)),
                            Right => beam.push(Some(Up)),
                        },
                        Mirror::BackSlash => match beam.direction {
                            Up => beam.push(Some(Left)),
                            Down => beam.push(Some(Right)),
                            Left => beam.push(Some(Up)),
                            Right => beam.push(Some(Down)),
                        },
                    },
                    Tile::Splitter(splitter) => match splitter {
                        Splitter::Horizontal if beam.direction.is_vertical() => {
                            new_beams.push(beam.split(Left));
                            new_beams.push(beam.split(Right));
                            // Discard original off to side to be cleaned up.
                            beam.position = Position { x: 0, y: 0 };
                        },
                        Splitter::Vertical if beam.direction.is_horizontal() => {
                            new_beams.push(beam.split(Up));
                            new_beams.push(beam.split(Down));
                            // Discard original off to side to be cleaned up.
                            beam.position = Position { x: 0, y: 0 };
                        },
                        _ => beam.push(None),
                    },
                }
            } else {
                beam.push(None);
            }
        }

        self.beams.append(&mut new_beams);
        // If new beams got spawned off the map they should get cleaned up here too.
        let remove: Vec<usize> = self
            .beams
            .iter()
            .enumerate()
            .filter(|(_index, beam)| {
                beam.position.is_out_of_bounds(&dimensions) || self.is_beam_repeating(beam)
            })
            .map(|(index, _beam)| index)
            // Reverse the list of indexes to remove: if we remove from the
            // beginning of the vector then the indexes after it are going to
            // shift. Always remove starting from highest index to lowest index.
            .rev()
            .collect();
        remove.into_iter().for_each(|index| {
            self.beams.remove(index);
        });
    }

    pub(crate) fn num_energized_tiles(&self) -> usize {
        self.energized.len()
    }

    #[cfg(debug_assertions)]
    fn beams_at_position(&self, position: &Position) -> Vec<&Beam> {
        self.beams.iter().filter(|beam| &beam.position == position).collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::TEST_INPUT;

    use super::*;

    #[test]
    fn test_max_position() {
        let contraption = Contraption::try_from(TEST_INPUT).unwrap();
        assert_eq!(10, contraption.get_dimensions().x);
        assert_eq!(10, contraption.get_dimensions().y);
    }

    #[test]
    fn test_stepping() {
        let mut contraption: Contraption = TEST_INPUT.try_into().unwrap();

        // After 10 steps, the light should have energized 11 tiles. It hits two
        // splitters (but one of them immediately falls off the map).
        (1..=10).for_each(|_| contraption.step());
        assert_eq!(11, contraption.num_energized_tiles());
        // After another 10 steps, the light should have energized 25 tiles.
        (1..=10).for_each(|_| contraption.step());
        assert_eq!(25, contraption.num_energized_tiles());
    }
}
