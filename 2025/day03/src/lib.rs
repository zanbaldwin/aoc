#[derive(Debug)]
pub enum Error {
    ParseError,
    EmptyBank,
}

pub type Bank = Vec<u8>;

// Old Part1 Code:
// /// The highest digit; if more than one, pick the most left-side.
// fn leftmost_largest(batteries: &[u8]) -> Result<(usize, u8), Error> {
//     batteries
//         .iter()
//         .enumerate()
//         .rev()
//         .reduce(|carry, item| if carry.1 <= item.1 { item } else { carry })
//         .map(|(position, power)| (position, *power))
//         .ok_or(Error::EmptyBank)
// }

// pub fn largest_from_two_digits(bank: &Bank) -> Result<u8, Error> {
//     if bank.len() < 2 {
//         return Err(Error::EmptyBank);
//     }
//     let batteries: Vec<u8> = bank.iter().map(|b| *b).collect();
//     let (position, first) = leftmost_largest(&batteries)?;
//     let is_last = position + 1 == batteries.len();
//     let batteries: &[u8] = if is_last { &batteries[..position] } else { &batteries[(position + 1)..] };
//     let (_, second) = leftmost_largest(batteries)?;
//     let result = if is_last { (second * 10) + first } else { (first * 10) + second };
//     Ok(result)
// }

pub fn largest_number_with_digits(bank: &Bank, digits: usize) -> Result<u64, Error> {
    if bank.len() < digits {
        return Err(Error::EmptyBank);
    }

    let mut removals = bank.len() - digits;
    let mut stack: Bank = Vec::with_capacity(bank.len());
    for &digit in bank {
        // If we still have removal allowance, compare the last digit added to
        // the stack to the one we're about to add, we may be able to replace instead.
        // But we can do that repeatedly until we reach a digit in the stack that is >=.
        while !stack.is_empty() && removals > 0 && stack.last().unwrap() < &digit {
            stack.pop();
            removals -= 1;
        }
        stack.push(digit);
    }

    // Stack may be over capacity, and we still need to remove digits.
    // But we're removing them from the end, so we ignore that part of the slice?
    // Even better, just take only the amount of digits we need.
    // Convert to u64 by using powers of 10.

    // Fun fact: Rust called the closure arguments "accumulator and element",
    // but I'm used to PHP-land where it's called "carry and item".
    let result = stack.iter().take(digits).fold(0u64, |carry, &item| (carry * 10) + (item as u64));
    Ok(result)
}

fn parse_bank(line: &str) -> Result<Bank, Error> {
    line.trim()
        .chars()
        .map(|c| c.to_digit(10).map(|d| d as u8).ok_or(Error::ParseError))
        .collect::<Result<_, _>>()
}
pub fn parse_banks(input: &str) -> Result<Vec<Bank>, Error> {
    input.trim().lines().map(|l| parse_bank(l)).collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;
    const TEST_INPUT: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[rstest]
    #[case("987654321111111", &[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1])]
    #[case("811111111111119", &[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9])]
    #[case("234234234234278", &[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8])]
    #[case("818181911112111", &[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1])]
    fn test_parse(#[case] input: &str, #[case] expected: &[u8]) {
        let bank: Bank = parse_bank(input).unwrap();
        assert_eq!(bank, expected);
    }

    #[rstest]
    #[case("987654321111111", 2, 98)]
    #[case("811111111111119", 2, 89)]
    #[case("234234234234278", 2, 78)]
    #[case("818181911112111", 2, 92)]
    #[case(
        "2231221233222124223231132223331221222232214218242422212242233231222321223132222232331333123342323342",
        2,
        84
    )]
    #[case(
        "5457444433566538437584354643454336277344655456233574464533634594453474664573725744796487654534744567",
        2,
        99
    )]
    #[case("987654321111111", 12, 987654321111)]
    #[case("811111111111119", 12, 811111111119)]
    #[case("234234234234278", 12, 434234234278)]
    #[case("818181911112111", 12, 888911112111)]
    fn test_largest_number_with_digits(#[case] input: &str, #[case] digits: usize, #[case] expected: u64) {
        let bank: Bank = parse_bank(input).unwrap();
        assert_eq!(expected, largest_number_with_digits(&bank, digits).unwrap());
    }

    #[test]
    fn test_input_parse() {
        let banks = parse_banks(TEST_INPUT).unwrap();
        assert_eq!(4, banks.len());
    }

    #[test]
    fn test_part1() {
        let banks = parse_banks(TEST_INPUT).unwrap();
        assert_eq!(357, banks.iter().map(|b| largest_number_with_digits(b, 2).unwrap()).sum::<u64>());
    }

    #[test]
    fn test_part2() {
        let banks = parse_banks(TEST_INPUT).unwrap();
        assert_eq!(3121910778619, banks.iter().map(|b| largest_number_with_digits(b, 12).unwrap()).sum::<u64>());
    }
}
