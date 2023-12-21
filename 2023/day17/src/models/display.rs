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

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c: char = match self.from.x.cmp(&self.to.x) {
            Ordering::Less => '>',
            Ordering::Greater => '<',
            Ordering::Equal => match self.from.y.cmp(&self.to.y) {
                Ordering::Less => 'v',
                Ordering::Greater => '^',
                Ordering::Equal => 'S',
            }
        };
        write!(f, "{c}")
    }
}

pub(crate) fn print_edges_taken(map: &Map, edges_taken: &[Edge]) {
    let mut output = String::new();
    for y in 1..=map.city.height {
        for x in 1..=map.city.width {
            let position = Position { x, y };
            if let Some(edge) = edges_taken.iter().find(|edge| edge.to == position) {
                output.push_str(edge.to_string().as_str());
            } else {
                output.push('.');
            }
        }
        output.push('\n');
    }
    println!("{}", output);
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
