use crate::{error::Error, hash_sum};

/// Process
///
/// Process the input, and provide an answer for Day 15 Part 1.
pub fn process(input: &str) -> Result<String, Error> {
    Ok(hash_sum(input.trim()).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1() {
        assert_eq!("1320", process(TEST_INPUT).unwrap());
    }
}
