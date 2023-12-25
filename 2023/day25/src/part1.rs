use crate::{cut_wires, error::Error, parser::parse};

pub fn process(input: &str) -> Result<String, Error> {
    let graph = parse(input)?;
    let (a, b) = cut_wires(&graph, 3)?;

    let product = a * b;
    Ok(product.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;

    #[test]
    fn test_part1() {
        assert_eq!("54", process(TEST_INPUT).unwrap());
    }
}
