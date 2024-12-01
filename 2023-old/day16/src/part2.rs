use crate::energize;
use crate::error::Error;
use crate::models::{Contraption, Direction, Position};
use rayon::prelude::*;

fn list_all_starting_points(contraption: &Contraption) -> Vec<(Position, Direction)> {
    use crate::models::Direction::*;
    let Position { x: width, y: height } = contraption.get_dimensions();
    let mut starting_points = Vec::with_capacity((width * 2) + (height * 2));
    (1..=width).for_each(|x| {
        starting_points.push((Position { x, y: 1 }, Down));
        starting_points.push((Position { x, y: height }, Up));
    });
    (1..=height).for_each(|y| {
        starting_points.push((Position { x: 1, y }, Right));
        starting_points.push((Position { x: width, y }, Left));
    });
    starting_points
}

pub fn process(input: &str) -> Result<String, Error> {
    let contraption: Contraption = input.try_into()?;
    let starting_points = list_all_starting_points(&contraption);
    let energy_levels: Vec<usize> = starting_points
        .par_iter()
        .filter_map(|start: &(Position, Direction)| -> Option<usize> {
            let mut contraption = contraption.clone();
            let (position, direction) = start.clone();
            contraption.initialize(position, direction).ok()?;
            Some(energize(&mut contraption).ok()?)
        })
        .collect();
    // There will always be an energy level of at one 1 because the
    // minimum size of the contraption is (1,1). Unwrap is fine.
    Ok(energy_levels.iter().max().unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use crate::TEST_INPUT;

    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!("51", process(TEST_INPUT).unwrap())
    }
}
