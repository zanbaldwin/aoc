use crate::{error::Error, models::Boxes};

pub fn process(input: &str) -> Result<String, Error> {
    let boxes: Boxes = input.try_into()?;
    let total_focusing_power: u32 = boxes.get_focusing_powers().values().sum();
    Ok(total_focusing_power.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part2() {
        assert_eq!("145", process(TEST_INPUT).unwrap());
    }
}
