use crate::input::Input;

pub trait Solution {
    type Error;
    fn parse(input: impl Input) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn part1(&self) -> Result<String, Self::Error>;

    fn part2(&self) -> Result<String, Self::Error>;
}
