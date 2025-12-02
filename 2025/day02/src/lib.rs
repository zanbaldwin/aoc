pub type Id = u64;

pub fn parse(input: &str) -> Vec<IdPair> {
    input.trim().split(',').map(|p| p.into()).collect()
}

#[derive(Debug, PartialEq)]
pub struct IdPair {
    left: Id,
    right: Id,
}
impl From<&str> for IdPair {
    fn from(value: &str) -> Self {
        let (left, right) = value.split_once('-').unwrap();
        let left: Id = left.parse().unwrap();
        let right: Id = right.parse().unwrap();
        Self {
            left: left.min(right),
            right: right.max(left),
        }
    }
}
impl IdPair {
    pub fn repeated_twice_ids(&self) -> Vec<Id> {
        let mut invalid = Vec::new();
        // Using the `itoa` crate made the repeated_any_ids() function faster than this one! We love the this crate!
        let mut buffer = itoa::Buffer::new();
        for id in self.left..=self.right {
            let string = buffer.format(id);
            let halfway = string.len() / 2;
            let (left, right) = (&string[..halfway], &string[halfway..]);
            if string.len().is_multiple_of(2) && left == right {
                invalid.push(id);
            }
        }
        invalid
    }

    pub fn repeated_any_ids(&self) -> Vec<Id> {
        let mut invalid = Vec::new();
        // Allocating a new string on each ID in the range, we're currently at ~130ms for this function (benchmarking
        // using official input). Let's try reducing the number of string allocations by using the `itoa` crate (brings
        // it down to ~75ms). Remember: there will always be people smarter than you, be smart by benefitting from their
        // hard work.
        let mut buffer = itoa::Buffer::new(); // (string representation of u64::MAX is 20 characters)
        'id: for id in self.left..=self.right {
            let string = buffer.format(id);
            let num_digits = string.len();
            let divisors = 1..=(num_digits / 2);
            for i in divisors.filter(|d| num_digits.is_multiple_of(*d)) {
                // The naïve way is: chunk = &string[..i]; test = chunk.repeat; if test == string...
                // BUT that cost me HUNDREDS... of milliseconds. Which, honestly, doesn't really matter.
                // BUT. I want that sweet, sweet blazingly fast Rusty goodness.
                // KILL ALL THE ALLOCATIONS (cuts execution time in half).
                if string.as_bytes().chunks(i).all(|chunk| chunk == &string.as_bytes()[..i]) {
                    invalid.push(id);
                    continue 'id;
                }
            }
        }
        invalid
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    const TEST_INPUT: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn test_parse() {
        let pairs = parse(TEST_INPUT);
        assert_eq!(11, pairs.len());
    }

    #[rstest]
    #[case("11-22", 11, 22)]
    #[case("95-115", 95, 115)]
    #[case("998-1012", 998, 1012)]
    #[case("1188511880-1188511890", 1188511880, 1188511890)]
    #[case("222220-222224", 222220, 222224)]
    #[case("1698522-1698528", 1698522, 1698528)]
    #[case("446443-446449", 446443, 446449)]
    #[case("38593856-38593862", 38593856, 38593862)]
    #[case("38593862-38593856", 38593856, 38593862)]
    fn test_from(#[case] value: &str, #[case] expected_left: Id, #[case] expected_right: Id) {
        let IdPair { left, right } = value.into();
        assert_eq!(expected_left, left);
        assert_eq!(expected_right, right);
    }

    #[rstest]
    #[case("11-22", &[11, 22])]
    #[case("95-115", &[99])]
    #[case("998-1012", &[1010])]
    #[case("1188511880-1188511890", &[1188511885])]
    #[case("222220-222224", &[222222])]
    #[case("1698522-1698528", &[])]
    #[case("446443-446449", &[446446])]
    #[case("38593856-38593862", &[38593859])]
    fn test_repeated_twice_ids(#[case] pair: &str, #[case] expected: &[Id]) {
        let pair: IdPair = pair.into();
        let invalid = pair.repeated_twice_ids();
        assert_eq!(expected, &invalid);
    }

    #[rstest]
    #[case("11-22", &[11, 22])]
    #[case("95-115", &[99, 111])]
    #[case("998-1012", &[999, 1010])]
    #[case("1188511880-1188511890", &[1188511885])]
    #[case("222220-222224", &[222222])]
    #[case("1698522-1698528", &[])]
    #[case("446443-446449", &[446446])]
    #[case("38593856-38593862", &[38593859])]
    #[case("565653-565659", &[565656])]
    #[case("824824821-824824827", &[824824824])]
    #[case("2121212118-2121212124", &[2121212121])]
    fn test_repeated_any_ids(#[case] pair: &str, #[case] expected: &[Id]) {
        let pair: IdPair = pair.into();
        let invalid = pair.repeated_any_ids();
        assert_eq!(expected, &invalid);
    }
}
