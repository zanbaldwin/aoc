use crate::{parse, Error};

pub fn process(input: &str) -> Result<String, Error> {
    let collection = parse(input)?;
    Ok(collection.ghost()?.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(
            "6",
            process(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            )
            .unwrap()
        );
    }
}
