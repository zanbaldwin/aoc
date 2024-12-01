use aoc_common::{ParseError, RunnerError, Solution};
use std::{collections::HashMap, fmt::Display};

pub struct Day01 {}
impl Solution for Day01 {
    type Parsed = (Vec<isize>, Vec<isize>);

    fn parse(&self, input: &str) -> Result<Self::Parsed, ParseError> {
        let mut first = Vec::new();
        let mut second = Vec::new();
        input.trim().lines().for_each(|line| {
            let mut words = line.split_whitespace();
            if let (Some(a), Some(b)) = (words.next(), words.next()) {
                if let (Ok(a), Ok(b)) = (a.parse(), b.parse()) {
                    first.push(a);
                    second.push(b);
                }
            }
        });
        Ok((first, second))
    }

    fn part1(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        let (mut first, mut second) = input;
        first.sort();
        second.sort();
        Ok(first.iter().zip(second).map(|(a, b)| (a - b).abs()).sum::<isize>())
    }

    fn part2(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        let (first, second) = input;
        // Iterate just once through the second list to calculate ahead of time
        // how many times each number appears, instead of performing the count
        // on each iteration for the numbers in the first list:
        // O(n) instead of O(n^2)
        let counts = second.iter().fold(HashMap::new(), |mut carry, item| {
            *carry.entry(*item).or_insert(0) += 1;
            carry
        });
        Ok(first.into_iter().map(|a| a * counts.get(&a).unwrap_or(&0)).sum::<isize>())
        // Note: I don't think there's much benefit due to the overhead of an
        // additional data structure; maybe better with bigger lists and more
        // repeating numbers. The following works just as well:
        // Ok(first.iter().map(|a| a * (second.iter().filter(|b| b == &a).count() as isize)).sum::<isize>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_parse() {
        let solution = Day01 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!((vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]), parsed);
    }

    #[test]
    fn test_part1_example() {
        let solution = Day01 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!("11".to_string(), solution.part1(parsed).unwrap().to_string());
    }

    #[test]
    fn test_part2_example() {
        let solution = Day01 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!("31".to_string(), solution.part2(parsed).unwrap().to_string());
    }
}
