use super::*;
use crate::part1::{Galaxy, Position};
use std::{collections::BTreeMap, fmt::Display};

impl Display for Galaxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Galaxy {
            id,
            position: (x, y),
            ..
        } = self;
        write!(f, "Galaxy: #{id} ({x}, {y})",)
    }
}

pub(crate) fn print_galaxies(galaxies: &[Galaxy]) {
    let galaxy_btree: BTreeMap<Position, Galaxy> = galaxies
        .iter()
        .map(|galaxy| (galaxy.position, *galaxy))
        .collect();
    let width: usize = galaxies
        .iter()
        .map(
            |Galaxy {
                 position: (x, _y), ..
             }| *x,
        )
        .max()
        .unwrap_or(0);
    let height: usize = galaxies
        .iter()
        .map(
            |Galaxy {
                 position: (_x, y), ..
             }| *y,
        )
        .max()
        .unwrap_or(0);

    for y in 1..=height {
        for x in 1..=width {
            match galaxy_btree.get(&(x, y)) {
                Some(_) => print!("#"),
                None => print!("."),
            }
        }
        println!();
    }
}
