use crate::aoc_error::AocError;
use std::iter;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let total: u32 = input.lines().map(|line: &str| -> u32 {
        let mut index = 0;
        let words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        let line_iter = iter::from_fn(move || {
            let check_line = &line[index..];
            let mut result: Option<char> = None;
            'found: for (i, word) in words.iter().enumerate() {
                if check_line.starts_with(word) {
                    result = Some((i + 1).to_string().chars().nth(0).unwrap());
                    break 'found;
                }
            }
            index += 1;
            result.or_else(|| check_line.chars().next())
        });
    
        let nums: Vec<u32> = line_iter.filter_map(|c| c.to_digit(10)).collect();
        (nums.first().unwrap() * 10) + nums.last().unwrap()
    }).sum();
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input: &'static str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!("281", process(input).unwrap());
    }

    #[test]
    fn test_word_order() {
        let line = "xtwone3four";
        assert_eq!("24", process(line).unwrap());
    }
}