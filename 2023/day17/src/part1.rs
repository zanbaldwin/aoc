use crate::error::Error;
use crate::models::City;

pub fn process(input: &str) -> Result<String, Error> {
    let mut map = City::try_from(input)?.map_bottom_right();
    let temperature_loss = map.find_path()?.get_temperature_loss();
    Ok(temperature_loss.to_string())
}

#[cfg(test)]
mod tests {
    use crate::TEST_INPUT;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!("102", process(TEST_INPUT).unwrap());
    }
}
