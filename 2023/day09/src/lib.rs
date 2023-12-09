use error::Error;

pub mod error;
pub mod part1;
pub mod part2;

type Reading = i64;
type PredictedReading = Reading;
type History = Vec<Reading>;
type Report = Vec<History>;

enum Direction {
    Forwards,
    Backwards,
}

trait Predictor {
    fn predict(&self, direction: Direction) -> Result<PredictedReading, Error>;
}
impl Predictor for History {
    fn predict(&self, direction: Direction) -> Result<PredictedReading, Error> {
        if self.len() == 1 {
            // Means we have encountered something like:
            // 1 3 8
            //  2 5
            //   3
            return Err(Error::NoAlgorithmicSequence);
        }
        let last = self.iter().last().ok_or(Error::NothingToPredict)?;
        let delta: History = self.windows(2).map(|chunk| chunk[1] - chunk[0]).collect();
        if delta.iter().all(|reading| reading == &0) {
            return Ok(*last);
        }
        Ok(match direction {
            Direction::Forwards => last + delta.predict(direction)?,
            Direction::Backwards => last - delta.predict(direction)?,
        })
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
