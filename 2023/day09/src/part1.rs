use crate::{error::Error, *};

pub fn process(input: &str) -> Result<String, Error> {
    let report = generate_report(input)?;
    let predictions: Result<Vec<PredictedReading>, Error> = report
        .iter()
        .map(|history| history.predict_forwards())
        .collect();
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
    fn test_part1() {
        assert_eq!("114", process(DAY09_TEST_INPUT).unwrap());
    }

    #[test]
    fn test_predict1() {
        assert_eq!(vec![0, 3, 6, 9, 12, 15].predict_forwards().unwrap(), 18);
    }

    #[test]
    fn test_predict2() {
        assert_eq!(vec![1, 3, 6, 10, 15, 21].predict_forwards().unwrap(), 28);
    }

    #[test]
    fn test_predict3() {
        assert_eq!(vec![10, 13, 16, 21, 30, 45].predict_forwards().unwrap(), 68);
    }
}
