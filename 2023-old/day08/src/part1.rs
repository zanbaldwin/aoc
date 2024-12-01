use crate::{parse, Error};

pub fn process(input: &str) -> miette::Result<String, Error> {
    let map_collection = parse(input)?;
    let count = map_collection.human()?;
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_a() {
        assert_eq!(
            "2",
            process(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            )
            .unwrap()
        );
    }

    #[test]
    fn test_part1_b() {
        assert_eq!(
            "6",
            process(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            )
            .unwrap()
        );
    }
}
