use crate::error::Error;
use crate::models::Contraption;

pub fn process(input: &str) -> Result<String, Error> {
    let mut contraption: Contraption = input.try_into()?;
    for _ in 1..=contraption.get_max_iterations() {
        if contraption.complete() {
            return Ok(contraption.energized_tiles().len().to_string());
        }
        contraption.step();
    }
    Err(Error::InfiniteLoop)
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
