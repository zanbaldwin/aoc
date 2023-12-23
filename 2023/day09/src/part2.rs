use crate::{error::Error, *};

pub fn process(input: &str) -> Result<String, Error> {
    let report = generate_report(input)?;
    let predictions: Result<Vec<PredictedReading>, Error> =
        report.iter().map(|history| history.predict(Direction::Backwards)).collect();
    let total: i64 = predictions?.iter().sum();
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY09_TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part2() {
        assert_eq!("2", process(DAY09_TEST_INPUT).unwrap());
    }

    #[test]
    fn test_predict1() {
        assert_eq!(vec![10, 13, 16, 21, 30, 45].predict(Direction::Backwards).unwrap(), 5);
    }

    #[test]
    fn test_small() {
        assert_eq!(vec![1, 2, 3].predict(Direction::Forwards).unwrap(), 4);
        assert_eq!(vec![1, 2].predict(Direction::Forwards).unwrap(), 3);
        // A single number should just repeat itself.
        assert_eq!(vec![1].predict(Direction::Forwards).unwrap(), 1);
        // Numbers all the same should also repeat.
        assert_eq!(vec![1, 1].predict(Direction::Forwards).unwrap(), 1);
        // No numbers should error.
        assert!(vec![].predict(Direction::Forwards).is_err());
    }
}
