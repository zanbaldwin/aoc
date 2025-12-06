mod parser;

use common::Solution;

#[derive(Debug)]
pub enum Error {
    InvalidOperation,
    Parser,
}

#[derive(Debug, PartialEq)]
struct Homework {
    problems: Vec<Calculation>,
}
#[derive(Debug, PartialEq)]
struct Calculation {
    numbers: Vec<u32>,
    operation: Operation,
}
impl Calculation {
    fn execute_human(&self) -> u64 {
        let iter = self.numbers.iter().map(|n| u64::from(*n));
        match self.operation {
            Operation::Addition => iter.sum(),
            Operation::Multiplication => iter.product(),
        }
    }
    fn execute_cephalopod(&self) -> u64 {
        todo!()
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
enum Operation {
    Addition,
    Multiplication,
}
impl TryFrom<&str> for Operation {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Self::Addition),
            "*" => Ok(Self::Multiplication),
            _ => Err(Error::InvalidOperation),
        }
    }
}

fn _is_column_separator(lines: &Vec<String>, i: usize) -> bool {
    lines.iter().all(|s| " " == s.get(i..=(i + 1)).unwrap_or(" "))
}

pub struct Day06 {
    homework: Homework,
}
impl Solution for Day06 {
    type Error = Error;
    fn parse(input: impl common::Input) -> Result<Self, Self::Error> {
        // let mut lines: Vec<String> = input.as_str().trim().lines().map(|s| s.to_string()).collect();
        // let operations: Vec<Operation> =
        //     lines.pop().unwrap().split_whitespace().map(|s| s.try_into().unwrap()).collect();
        // let length = lines.iter().map(|l| l.len()).max().unwrap_or(0);
        // let mut calculations: Vec<Calculation> = Vec::with_capacity(lines.len());
        // let mut numbers: Vec<char> = Vec::new();
        // for i in 0..length {
        //     // Go one column at a time, saving the string representation for
        //     // each number until we hit a column boundary, where we save
        //     // those numbers as a Calculation.
        // }

        let (_remaining, homework) = parser::parse(input.as_str()).map_err(|_| Error::Parser)?;
        Ok(Self { homework })
    }

    fn part1(&self) -> Result<String, Self::Error> {
        let result_sum: u64 = self.homework.problems.iter().map(|calc| calc.execute_human()).sum();
        Ok(result_sum.to_string())
    }
    fn part2(&self) -> Result<String, Self::Error> {
        let result_sum: u64 = self.homework.problems.iter().map(|calc| calc.execute_cephalopod()).sum();
        Ok(result_sum.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::input::RawInput;
    const TEST_INPUT: &str = r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +"#;

    #[test]
    fn test_parse() {
        let input = RawInput::new(TEST_INPUT);
        let solution = Day06::parse(input).unwrap();
        assert_eq!(
            solution.homework,
            Homework {
                problems: vec![
                    Calculation {
                        numbers: vec![123, 45, 6],
                        operation: Operation::Multiplication,
                    },
                    Calculation {
                        numbers: vec![328, 64, 98],
                        operation: Operation::Addition,
                    },
                    Calculation {
                        numbers: vec![51, 387, 215],
                        operation: Operation::Multiplication,
                    },
                    Calculation {
                        numbers: vec![64, 23, 314],
                        operation: Operation::Addition,
                    }
                ]
            }
        )
    }

    #[test]
    fn test_part1() {
        let input = RawInput::new(TEST_INPUT);
        let solution = Day06::parse(input).unwrap();
        assert_eq!("4277556", solution.part1().unwrap());
    }

    #[test]
    fn test_part2() {
        let input = RawInput::new(TEST_INPUT);
        let solution = Day06::parse(input).unwrap();
        assert_eq!("3263827", solution.part2().unwrap());
    }
}
