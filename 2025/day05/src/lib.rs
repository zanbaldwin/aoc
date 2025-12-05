mod parser;

use std::ops::RangeInclusive;

use common::Solution;

#[derive(Debug)]
pub enum Error {
    Nom,
}

type Ingredient = u64;
#[derive(Debug, PartialEq)]
struct Inventory {
    fresh: Vec<RangeInclusive<Ingredient>>,
    available: Vec<Ingredient>,
}
impl Inventory {
    fn new(fresh: Vec<RangeInclusive<Ingredient>>, available: Vec<Ingredient>) -> Self {
        Self {
            fresh: OverlappingRanges::new(fresh).combine(),
            available,
        }
    }

    fn is_fresh(&self, ingredient: Ingredient) -> bool {
        self.fresh.iter().any(|range| range.contains(&ingredient))
    }
}

pub struct Day05 {
    inventory: Inventory,
}
impl Solution for Day05 {
    type Error = Error;
    fn parse(input: impl common::Input) -> Result<Self, Self::Error> {
        let (_, inventory) = parser::parse(input.as_str()).map_err(|_| Error::Nom)?;
        Ok(Self { inventory })
    }

    fn part1(&self) -> Result<String, Self::Error> {
        let available_fresh =
            self.inventory.available.iter().filter(|ingredient| self.inventory.is_fresh(**ingredient)).count();
        Ok(available_fresh.to_string())
    }
    fn part2(&self) -> Result<String, Self::Error> {
        let num: usize = self.inventory.fresh.iter().map(|r| r.clone().count()).sum();
        Ok(num.to_string())
    }
}

struct OverlappingRanges<T: Ord + Copy> {
    ranges: Vec<RangeInclusive<T>>,
}
impl<T: Ord + Copy> OverlappingRanges<T> {
    fn new(ranges: Vec<RangeInclusive<T>>) -> Self {
        Self { ranges }
    }

    fn does_range_overlap(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool {
        a.start() <= b.end() && b.start() <= a.end()
    }

    fn combine_two_ranges(a: RangeInclusive<T>, b: RangeInclusive<T>) -> RangeInclusive<T> {
        *a.start().min(b.start())..=*a.end().max(b.end())
    }

    fn combine_ranges(mut ranges: Vec<RangeInclusive<T>>) -> Vec<RangeInclusive<T>> {
        if ranges.len() == 0 {
            return Vec::with_capacity(0);
        }
        // Sort by range start.
        ranges.sort_by(|a, b| a.start().cmp(b.start()));
        ranges.rotate_left(1);
        // Guaranteed to have at least one element.
        let mut prev = ranges.pop().unwrap();
        let mut combined = vec![];
        for current in ranges.into_iter() {
            if Self::does_range_overlap(&current, &prev) {
                prev = Self::combine_two_ranges(current, prev);
            } else {
                combined.push(prev);
                prev = current;
            }
        }
        combined.push(prev);
        combined
    }

    fn combine(self) -> Vec<RangeInclusive<T>> {
        Self::combine_ranges(self.ranges)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::input::RawInput;
    use rstest::rstest;
    const TEST_INPUT: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    #[rstest]
    #[case(1..=10, 12..=20, false)]
    #[case(1..=10, 11..=20, false)]
    #[case(1..=10, 10..=20, true)]
    #[case(1..=10, 1..=10, true)]
    #[case(1..=10, 8..=11, true)]
    #[case(1..=10, 8..=9, true)]
    #[case(1..=11, 12..=20, false)]
    #[case(1..=12, 12..=20, true)]
    #[case(1..=13, 12..=20, true)]
    #[case(10..=1, 12..=20, false)]
    fn test_does_range_overlap(#[case] a: RangeInclusive<u64>, #[case] b: RangeInclusive<u64>, #[case] expected: bool) {
        assert_eq!(expected, OverlappingRanges::does_range_overlap(&a, &b));
    }

    #[rstest]
    #[case(1..=10, 10..=20, 1..=20)]
    #[case(1..=10, 1..=10, 1..=10)]
    #[case(1..=10, 8..=11, 1..=11)]
    #[case(1..=10, 8..=9, 1..=10)]
    #[case(1..=12, 12..=20, 1..=20)]
    #[case(1..=13, 12..=20, 1..=20)]
    fn test_combine_range(
        #[case] a: RangeInclusive<u64>,
        #[case] b: RangeInclusive<u64>,
        #[case] expected: RangeInclusive<u64>,
    ) {
        assert_eq!(expected, OverlappingRanges::combine_two_ranges(a, b));
    }

    #[rstest]
    #[case(vec![1..=5, 10..=15], vec![1..=5, 10..=15])]
    #[case(vec![1..=5, 5..=10], vec![1..=10])]
    #[case(vec![1..=5, 6..=10], vec![1..=5, 6..=10])]
    #[case(vec![1..=5, 4..=10, 9..=15], vec![1..=15])]
    #[case(vec![10..=15, 6..=11], vec![6..=15])]
    #[case(vec![1..=5, 15..=20, 4..=6, 13..=15, 9..=11], vec![1..=6, 9..=11, 13..=20])]
    fn test_combine_ranges(#[case] ranges: Vec<RangeInclusive<u64>>, #[case] expected: Vec<RangeInclusive<u64>>) {
        assert_eq!(expected, OverlappingRanges::combine_ranges(ranges));
    }

    #[test]
    fn test_parse() {
        let input = RawInput::new(TEST_INPUT);
        let solution = Day05::parse(input).unwrap();
        let expected = Inventory::new(vec![3..=5, 10..=14, 16..=20, 12..=18], vec![1, 5, 8, 11, 17, 32]);
        assert_eq!(solution.inventory, expected);
    }

    #[test]
    fn test_part1() {
        let input = RawInput::new(TEST_INPUT);
        let solution = Day05::parse(input).unwrap();
        assert_eq!("3", solution.part1().unwrap());
    }

    #[test]
    fn test_part2() {
        let input = RawInput::new(TEST_INPUT);
        let solution = Day05::parse(input).unwrap();
        assert_eq!("14", solution.part2().unwrap());
    }
}
