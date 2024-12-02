use aoc_common::{ParseError, RunnerError, Solution};
use std::{fmt::Display, num::ParseIntError};

enum Safety {
    Increasing,
    Decreasing,
    Unsafe,
}

pub struct Day02 {}
impl Solution for Day02 {
    type Parsed = Vec<Vec<isize>>;

    fn parse(&self, input: &str) -> Result<Self::Parsed, ParseError> {
        input
            .trim()
            .lines()
            .map(|line| line.split_whitespace().map(|n| n.parse()).collect::<Result<Vec<_>, ParseIntError>>())
            .collect::<Result<Vec<_>, ParseIntError>>()
            .map_err(ParseError::Int)
    }

    fn part1(&self, reports: Self::Parsed) -> Result<impl Display, RunnerError> {
        Ok(reports
            .iter()
            .filter_map(|levels| get_tolerant_report(levels, false).ok())
            .filter(|report| !matches!(report, Safety::Unsafe))
            .count())
    }

    fn part2(&self, reports: Self::Parsed) -> Result<impl Display, RunnerError> {
        Ok(reports
            .iter()
            .filter_map(|levels| {
                match get_tolerant_report(levels, true) {
                    // Okay it doesn't work if the first level is the one to remove to make it safe.
                    // Could code that into my solution, or I could just run it again with the first one removed.
                    Ok(Safety::Unsafe) => get_tolerant_report(&levels[1..], false).ok(),
                    Ok(safety) => Some(safety),
                    Err(_) => None,
                }
            })
            .filter(|report| !matches!(report, Safety::Unsafe))
            .count())
    }
}

fn get_tolerant_report(levels: &[isize], tolerate_single_fault: bool) -> Result<Safety, RunnerError> {
    let mut iter = levels.iter();
    let mut has_tolerated_fault = !tolerate_single_fault;
    let Some(mut previous) = iter.next() else {
        return Err(RunnerError::Custom("no levels in report".to_string()));
    };
    let mut result: Option<Safety> = None;
    'levels: for level in iter {
        let correct_direction = match result {
            None if level != previous => true,
            Some(Safety::Increasing) => previous < level,
            Some(Safety::Decreasing) => previous > level,
            _ => false,
        };

        if !correct_direction || (level - previous).abs() > 3 {
            if !has_tolerated_fault {
                has_tolerated_fault = true;
                continue 'levels;
            } else {
                return Ok(Safety::Unsafe);
            }
        }

        if result.is_none() {
            result = if level > previous { Some(Safety::Increasing) } else { Some(Safety::Decreasing) };
        }
        previous = level;
    }

    result.ok_or_else(|| RunnerError::Custom("Report not generated.".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_parse() {
        let solution = Day02 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!(
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ],
            parsed
        );
    }

    #[test]
    fn test_part_one() {
        let solution = Day02 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!("2", solution.part1(parsed).unwrap().to_string());
    }

    #[rstest]
    #[case("1 2 3 4 5", true)]
    #[case("1 1 2 3 4", false)]
    #[case("1 2 3 3 4", false)]
    #[case("74 77 81 82 83 87", false)]
    #[case("76 77 79 80 84 83 84", false)]
    #[case("66 68 69 72 75", true)]
    fn test_part_one_combinations(#[case] report: &str, #[case] valid: bool) {
        let solution = Day02 {};
        let parsed = solution.parse(report).unwrap();
        assert_eq!(if valid { "1" } else { "0" }, solution.part1(parsed).unwrap().to_string());
    }

    #[test]
    fn test_part_two() {
        let solution = Day02 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!("4", solution.part2(parsed).unwrap().to_string());
    }

    #[rstest]
    #[case("1 2 3 4 5", true)]
    #[case("1 1 2 3 4", true)]
    #[case("1 1 1 2 3", false)]
    #[case("1 2 3 4 4 5", true)]
    #[case("1 2 3 4 4 4", false)]
    #[case("74 77 81 82 83 87", false)]
    #[case("76 77 79 80 84 83 84", true)]
    #[case("66 68 69 72 75", true)]
    #[case("66 68 69 72 71", true)]
    #[case("10 1 2 3 4 5", true)]
    fn test_part_two_combinations(#[case] report: &str, #[case] valid: bool) {
        let solution = Day02 {};
        let parsed = solution.parse(report).unwrap();
        assert_eq!(if valid { "1" } else { "0" }, solution.part2(parsed).unwrap().to_string());
    }

    #[rstest]
    fn test_part_of_real_input() {
        let input = "94 96 93 95 98
77 79 77 78 75 72 72
23 24 21 19 20 19 15
32 34 33 34 27
15 17 17 16 15
34 37 35 32 31 31 33
24 25 22 20 19 19 19
15 18 15 15 13 11 7
88 90 90 89 86 85 83 76
81 84 81 77 74 72 69
50 53 50 46 49
51 53 49 48 48
52 53 49 46 44 41 38 34
57 60 58 55 51 48 41
87 89 88 82 79
64 65 64 61 54 55
95 96 93 90 83 81 78 78
17 20 18 11 9 6 5 1
42 45 38 36 29
48 48 47 44 41 40";
        let solution = Day02 {};
        let parsed = solution.parse(input).unwrap();
        assert_eq!("1", solution.part2(parsed).unwrap().to_string());
    }
}
