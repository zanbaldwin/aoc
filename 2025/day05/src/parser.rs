use std::ops::RangeInclusive;

use crate::Inventory;
use nom::{
    IResult, Parser, bytes::complete::tag, character::complete::u64 as parse_u64, multi::separated_list1,
    sequence::separated_pair,
};

pub(crate) fn parse(input: &str) -> IResult<&str, Inventory> {
    separated_pair(parse_ranges, tag("\n\n"), parse_ingredients)
        .map(|(ranges, ingredients)| Inventory::new(ranges, ingredients))
        .parse(input)
}

fn parse_ranges(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    separated_list1(tag("\n"), parse_range).parse(input)
}

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    separated_pair(parse_ingredient, tag("-"), parse_ingredient)
        .map(|(from, to)| from.min(to)..=to.max(from))
        .parse(input)
}

fn parse_ingredients(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag("\n"), parse_ingredient).parse(input)
}

fn parse_ingredient(input: &str) -> IResult<&str, u64> {
    parse_u64.parse(input)
}
