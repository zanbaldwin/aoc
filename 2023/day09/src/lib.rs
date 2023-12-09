use error::Error;

pub mod error;
pub mod part1;
pub mod part2;

type Reading = i64;
type PredictedReading = Reading;
type History = Vec<Reading>;
type Report = Vec<History>;

trait Predictor {
    fn predict_forwards(&self) -> Result<PredictedReading, Error>;
    fn predict_backwards(&self) -> Result<PredictedReading, Error>;
}
impl Predictor for History {
    fn predict_forwards(&self) -> Result<PredictedReading, Error> {
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
            delta.predict_forwards()?
        };
        Ok(last + prediction)
    }

    fn predict_backwards(&self) -> Result<PredictedReading, Error> {
        if self.len() == 1 {
            // Means we have encountered something like:
            // 1 3 8
            //  2 5
            //   3
            return Err(Error::NoAlgorithmicSequence);
        }
        let first = self.iter().next().ok_or(Error::NothingToPredict)?;
        let delta: History = self.windows(2).map(|chunk| chunk[1] - chunk[0]).collect();
        let prediction = if delta.iter().all(|reading| reading == &0) {
            // If we have a delta of all zeros we already know the result will
            // be zero regardless of how many iterations to remove the zeros one
            // by one.
            0
        } else {
            delta.predict_backwards()?
        };
        Ok(first - prediction)
    }
}

pub(crate) fn generate_report(input: &str) -> Result<Report, Error> {
    input
        .trim()
        .lines()
        .map(|line| -> Result<History, Error> {
            line.split_whitespace()
                .map(|num| num.parse().map_err(Error::InvalidNumber))
                .collect()
        })
        .collect()
}
