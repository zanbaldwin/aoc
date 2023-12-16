use std::fmt::{Display, Formatter};

use crate::models::*;

trait ToChar {
    fn to_char(&self) -> char;
}
impl ToChar for Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::Mirror(mirror) => mirror.to_char(),
            Tile::Splitter(splitter) => splitter.to_char(),
        }
    }
}
impl ToChar for Splitter {
    fn to_char(&self) -> char {
        match self {
            Splitter::Horizontal => '-',
            Splitter::Vertical => '|',
        }
    }
}
impl ToChar for Mirror {
    fn to_char(&self) -> char {
        match self {
            Mirror::ForwardSlash => '/',
            Mirror::BackSlash => '\\',
        }
    }
}
impl ToChar for Direction {
    fn to_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
impl Display for Splitter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
impl Display for Mirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Display for Contraption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for y in 1..=self.height {
            for x in 1..=self.width {
                let position = Position { x, y };
                let beams_at_position = self.beams_at_position(&position);
                if !beams_at_position.is_empty() {
                    result.push(match beams_at_position.len() {
                        1 => beams_at_position[0].direction.to_char(),
                        n if n < 10 => char::from_digit(n as u32, 10).unwrap(),
                        _ => '*',
                    });
                } else if let Some(tile) = self.tiles.get(&position) {
                    result.push(tile.to_char());
                } else if self.energized.get(&position).is_some() {
                    result.push('#');
                } else {
                    result.push('.');
                }
            }
            result.push('\n');
        }
        write!(f, "{result}")
    }
}

#[cfg(test)]
mod tests {
    use crate::{models::Contraption, TEST_INPUT};

    #[test]
    fn test_display() {
        let mut contraption: Contraption = TEST_INPUT.try_into().unwrap();
        // Remove the initialized beam so that the display output looks like the
        // original input.
        contraption.beams = vec![];
        assert_eq!(TEST_INPUT.trim(), contraption.to_string().trim());
    }
}
