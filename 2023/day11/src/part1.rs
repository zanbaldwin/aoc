use crate::{debug::print_galaxies, error::Error};
use std::collections::BTreeMap;

pub fn process(input: &str) -> Result<String, Error> {
    println!("Initial Galactic Map:");
    print_galaxies(&galaxies);

    // Let's do lengthways first.
    let mut rows_containing_no_galaxies: Vec<usize> = Vec::new();
    for line in 1..=height {
        if galaxies
            .iter()
            .filter(
                |Galaxy {
                     position: (_x, y), ..
                 }| y == &line,
            )
            .count()
            == 0
        {
            rows_containing_no_galaxies.push(line);
        }
    }

    let mut columns_containing_no_galaxies: Vec<usize> = Vec::new();
    for column in 1..=width {
        if galaxies
            .iter()
            .filter(
                |Galaxy {
                     position: (x, _y), ..
                 }| x == &column,
            )
            .count()
            == 0
        {
            columns_containing_no_galaxies.push(column);
        }
    }

    columns_containing_no_galaxies.iter().for_each(|column| {
        galaxies
            .iter_mut()
            .filter(
                |Galaxy {
                     position: (x, _y), ..
                 }| x > column,
            )
            .for_each(
                |Galaxy {
                     position: (x, _y), ..
                 }| *x += 1,
            );
    });
    rows_containing_no_galaxies.iter().for_each(|row| {
        galaxies
            .iter_mut()
            .filter(
                |Galaxy {
                     position: (_x, y), ..
                 }| y > row,
            )
            .for_each(
                |Galaxy {
                     position: (_x, y), ..
                 }| *y += 1,
            );
    });

    // columns_containing_no_galaxies.iter().for_each(|column| {
    //     rows_containing_no_galaxies.iter().for_each(|row| {
    //         galaxies
    //             .iter_mut()
    //             .filter(
    //                 |Galaxy {
    //                      position: (x, y), ..
    //                  }| x > column && y > row,
    //             )
    //             .for_each(
    //                 |Galaxy {
    //                      position: (x, y), ..
    //                  }| {
    //                     *x += 1;
    //                     *y += 1
    //                 },
    //             );
    //     });
    // });

    println!("Expanded Galactic Map:");
    print_galaxies(&galaxies);

    // let galaxy_map: BTreeMap<Position, Galaxy> = galaxies
    //     .iter()
    //     .map(|galaxy| (galaxy.position, *galaxy))
    //     .collect();

    // let num_galaxies = galaxies.len();
    // let num_galactic_pairs = (num_galaxies * (num_galaxies - 1)) / 2;

    Err(Error::NotYetImplemented)
}

#[cfg(test)]
mod tests {
    use super::*;

    const GALACTICAL_MAP: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part1() {
        assert_eq!("374", process(GALACTICAL_MAP).unwrap());
    }

    #[test]
    fn test_expansion() {}
}
