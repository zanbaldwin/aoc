use common::Solution;
use common::grid::{Grid, Position};

#[derive(Debug)]
pub enum Error {
    InvalidSymbol,
}

#[derive(Clone)]
pub enum Item {
    PaperRoll,
    Empty,
}

pub struct Day04 {
    grid: Grid<Item>,
}
impl Day04 {
    // Fewer than 4: at most 3.
    const AT_MOST: usize = 3;

    fn grid_symbol(char: char) -> Result<Item, Error> {
        match char {
            '@' => Ok(Item::PaperRoll),
            '.' => Ok(Item::Empty),
            _ => Err(Error::InvalidSymbol),
        }
    }

    fn get_removable(grid: &Grid<Item>, surrounded_at_most: usize) -> Vec<(&Position, &Item)> {
        grid.iter()
            .filter(|(position, item)| {
                matches!(item, Item::PaperRoll)
                    && grid
                        .get_adjacent(position)
                        .iter()
                        .filter(|(_position, item)| matches!(item, Item::PaperRoll))
                        .count()
                        <= surrounded_at_most
            })
            .collect()
    }
}
impl Solution for Day04 {
    type Error = Error;
    fn from_input(input: impl common::Input) -> Result<Self, Self::Error> {
        let grid = Grid::from_text(input.into_string().as_ref(), Self::grid_symbol)?;
        Ok(Self { grid })
    }

    fn part1(&self) -> Result<String, Self::Error> {
        let count = Self::get_removable(&self.grid, Self::AT_MOST).len();
        Ok(count.to_string())
    }
    fn part2(&self) -> Result<String, Self::Error> {
        // The solution parts are (intentionally) meant to be immutable/idempotent,
        // but in order to calculate the answer we need to keep track of state:
        // clone the grid, despite it being a complete waste of allocation.
        let mut grid = self.grid.clone();
        let mut total = 0;
        'remove: loop {
            let removable: Vec<Position> = Self::get_removable(&grid, Self::AT_MOST)
                .into_iter()
                .map(|(position, _item)| position)
                .cloned()
                .collect();
            let count = removable.len();
            if count == 0 {
                break 'remove;
            }
            total += count;
            for position in removable {
                grid.remove(&position);
            }
        }
        Ok(total.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::input::RawInput;
    const TEST_INPUT: &str = r#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn test_part1() {
        let input = RawInput::new(TEST_INPUT);
        let solution = Day04::from_input(input).unwrap();
        assert_eq!("13", solution.part1().unwrap());
    }

    #[test]
    fn test_part2() {
        let input = RawInput::new(TEST_INPUT);
        let solution = Day04::from_input(input).unwrap();
        assert_eq!("43", solution.part2().unwrap());
    }
}
