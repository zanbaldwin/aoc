use crate::error::Error;

type Reading = i64;
type PredictedReading = Reading;
type History = Vec<Reading>;
type Report = Vec<History>;

trait Predictor {
    fn predict(&self) -> Result<PredictedReading, Error>;
}
impl Predictor for History {
    fn predict(&self) -> Result<PredictedReading, Error> {
        if self.len() == 1 {
            // Means we have encountered something like:
            // 1 3 8
            //  2 5
            //   3
            return Err(Error::NoAlgorithmicSequence);
        }
        let last = self.iter().last().ok_or(Error::NothingToPredict)?;
        let delta: History = self.windows(2).map(|chunk| chunk[1] - chunk[0]).collect();
        let prediction = if delta.iter().all(|reading| reading == &0) {
            // If we have a delta of all zeros we already know the result will
            // be zero regardless of how many iterations to remove the zeros one
            // by one.
            0
        } else {
            delta.predict()?
        };
        Ok(last + prediction)
    }
}

pub fn process(input: &str) -> Result<String, Error> {
    let report: Result<Report, Error> = input
        .trim()
        .lines()
        .map(|line| -> Result<History, Error> {
            line.split_whitespace()
                .map(|num| -> Result<Reading, Error> { num.parse().map_err(Error::InvalidNumber) })
                .collect()
        })
        .collect();

    let predictions: Result<Vec<PredictedReading>, Error> =
        report?.iter().map(|history| history.predict()).collect();
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
        assert_eq!(vec![0, 3, 6, 9, 12, 15].predict().unwrap(), 18);
    }

    #[test]
    fn test_predict2() {
        assert_eq!(vec![1, 3, 6, 10, 15, 21].predict().unwrap(), 28);
    }

    #[test]
    fn test_predict3() {
        assert_eq!(vec![10, 13, 16, 21, 30, 45].predict().unwrap(), 68);
    }
}
