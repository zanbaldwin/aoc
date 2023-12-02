use crate::aoc_error::AocError;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let total: u32 = input.lines().map(|line: &str| -> u32 {
        let nums: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        (nums.first().unwrap() * 10) + nums.last().unwrap()
    }).sum();
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: &'static str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!("142", process(input).unwrap());
    }
}