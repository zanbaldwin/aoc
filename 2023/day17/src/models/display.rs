use std::fmt::{Debug, Display, Formatter};

use super::*;

impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for y in 1..=self.height {
            for x in 1..=self.width {
                let position = Position { x, y };
                if let Some(block) = self.blocks.get(&position) {
                    output.push_str(&block.temperature_loss.to_string()[0..1]);
                } else {
                    panic!("City incorrectly constructed.");
                }
            }
            output.push('\n');
        }
        write!(f, "{output}")
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

#[cfg(test)]
mod tests {
    use crate::models::City;
    use crate::TEST_INPUT;

    #[test]
    fn test_parse_and_display() {
        let city = City::try_from(TEST_INPUT).unwrap();
        assert_eq!(TEST_INPUT.trim(), city.to_string().trim());
    }
}
