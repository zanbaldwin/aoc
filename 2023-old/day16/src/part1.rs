use crate::energize;
use crate::error::Error;
use crate::models::Contraption;

pub fn process(input: &str) -> Result<String, Error> {
    let mut contraption: Contraption = input.try_into()?;
    let num_energized_tiles = energize(&mut contraption)?;
    Ok(num_energized_tiles.to_string())
}

#[cfg(test)]
mod tests {
    use crate::TEST_INPUT;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!("46", process(TEST_INPUT).unwrap());
    }
}
