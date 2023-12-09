use crate::{error::Error, *};

pub fn process(input: &str) -> Result<String, Error> {
    let report = generate_report(input)?;
    let predictions: Result<Vec<PredictedReading>, Error> = report
        .iter()
        .map(|history| history.predict(Direction::Backwards))
        .collect();
    let total: i64 = predictions?.iter().sum();
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict1() {
        assert_eq!(
            vec![10, 13, 16, 21, 30, 45]
                .predict(Direction::Backwards)
                .unwrap(),
            5
        );
    }
}
