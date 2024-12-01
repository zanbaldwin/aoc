use crate::models::{Galaxy, Position, Spacing, Universe};
use std::{collections::BTreeSet, fmt::Display};

impl Display for Galaxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Galaxy {
            id,
            position: Position { x, y },
            ..
        } = self;
        write!(f, "Galaxy: #{id} ({x}, {y})",)
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let galaxies: BTreeSet<Position> =
            self.galaxies.iter().map(|galaxy| galaxy.position).collect();
        let mut result = String::new();
        for y in 1..=self.height {
            for x in 1..=self.width {
                if galaxies.contains(&Position { x, y }) {
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

impl<'a> Display for Spacing<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result =
            String::with_capacity((self.universe.width + 2) * (self.universe.height + 1));
        let mut top = String::new();
        for column in 0..=self.universe.width {
            if self.columns.contains(&column) {
                top.push('v');
            } else {
                top.push(' ');
            }
        }
        result.push_str(top.trim_end());
        result.push('\n');
        let universe = format!("{}", self.universe);
        universe
            .trim()
            .lines()
            .enumerate()
            .for_each(|(index, line)| {
                if self.rows.contains(&(index + 1)) {
                    result.push('>');
                } else {
                    result.push(' ');
                }
                result.push_str(line);
                result.push('\n');
            });
        write!(f, "{result}")
    }
}
