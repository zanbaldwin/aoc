use crate::error::Error;
use crate::models::Contraption;

pub mod error;
pub(crate) mod models;
pub mod part1;
pub mod part2;

#[cfg(test)]
const TEST_INPUT: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

fn energize(contraption: &mut Contraption) -> Result<usize, Error> {
    for _ in 1..=contraption.get_max_iterations() {
        if contraption.complete() {
            return Ok(contraption.num_energized_tiles());
        }
        contraption.step();
    }
    Err(Error::InfiniteLoop)
}
