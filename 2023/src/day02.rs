use aoc_common::{ParseError, RunnerError, Solution};
use std::fmt::Display;

static ELF_RESTRICTION: models::Counts = models::Counts { red: 12, green: 13, blue: 14 };

pub struct Day02 {}
impl Solution for Day02 {
    type Parsed = Vec<models::Game>;

    fn parse(&self, input: &str) -> Result<Self::Parsed, ParseError> {
        parser::parse(input)
    }

    fn part1(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        Ok(input
            .iter()
            .filter(|game| !game.get_rounds().iter().any(|round| !round.get_counts().possible_with(&ELF_RESTRICTION)))
            .map(|game| game.get_id() as u32)
            .sum::<u32>())
    }

    fn part2(&self, input: Self::Parsed) -> Result<impl Display, RunnerError> {
        Ok(input
            .iter()
            .map(|game| game.get_minimum_counts())
            .map(|counts| counts.red as u32 * counts.green as u32 * counts.blue as u32)
            .sum::<u32>())
    }
}

mod models {
    use std::cmp::max;

    #[derive(Debug, PartialEq, Clone)]
    pub(crate) enum Colour {
        Red,
        Blue,
        Green,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub(crate) struct Draw {
        colour: Colour,
        amount: u16,
    }
    #[derive(Debug, PartialEq, Default)]
    pub(crate) struct Counts {
        pub red: u16,
        pub green: u16,
        pub blue: u16,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub(crate) struct Round {
        draws: Vec<Draw>,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Game {
        id: u16,
        rounds: Vec<Round>,
    }

    impl Colour {
        fn from_str(colour: &str) -> Self {
            match colour {
                "red" => Self::Red,
                "green" => Self::Green,
                "blue" => Self::Blue,
                // Can't be bothered to figure out proper error handling from here.
                _ => panic!("Incorrect colour \'{colour}\' specified."),
            }
        }
    }

    impl Draw {
        pub fn from_enum(amount: u16, colour: Colour) -> Self {
            Self { amount, colour }
        }
        pub fn from_str(amount: u16, colour: &str) -> Self {
            Self::from_enum(amount, Colour::from_str(colour))
        }
    }

    impl Counts {
        pub fn possible_with(&self, restriction: &Counts) -> bool {
            self.red <= restriction.red && self.green <= restriction.green && self.blue <= restriction.blue
        }
    }

    impl Round {
        pub(crate) fn new(draws: Vec<Draw>) -> Self {
            Self { draws }
        }

        pub(crate) fn count_for(&self, colour: Colour) -> u16 {
            self.draws.iter().filter(|draw| draw.colour == colour).map(|draw| draw.amount).sum()
        }

        pub(crate) fn get_counts(&self) -> Counts {
            Counts {
                red: self.count_for(Colour::Red),
                green: self.count_for(Colour::Green),
                blue: self.count_for(Colour::Blue),
            }
        }
    }

    impl Game {
        pub(crate) fn new(id: u16, rounds: Vec<Round>) -> Self {
            Self { id, rounds }
        }

        pub(crate) fn get_id(&self) -> u16 {
            self.id
        }

        pub(crate) fn get_rounds(&self) -> Vec<Round> {
            self.rounds.clone()
        }

        pub(crate) fn get_minimum_counts(&self) -> Counts {
            self.rounds
                .iter()
                .map(|round| round.get_counts())
                .reduce(|acc, counts| Counts {
                    red: max(acc.red, counts.red),
                    green: max(acc.green, counts.green),
                    blue: max(acc.blue, counts.blue),
                })
                .unwrap_or_default()
        }
    }
}

mod parser {
    use super::models::{Draw, Game, Round};
    use aoc_common::ParseError;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{digit1, line_ending, multispace1, space0},
        combinator::{map, map_res},
        multi::separated_list1,
        sequence::tuple,
        IResult,
    };

    fn parse_draw(input: &str) -> IResult<&str, Draw> {
        map(
            tuple((
                map_res(digit1, |s: &str| s.parse::<u16>()),
                multispace1,
                alt((tag("red"), tag("green"), tag("blue"))),
            )),
            |(amount, _, colour): (u16, &str, &str)| Draw::from_str(amount, colour),
        )(input)
    }

    fn parse_round(input: &str) -> IResult<&str, Round> {
        map(separated_list1(tuple((tag(","), space0)), parse_draw), Round::new)(input)
    }

    fn parse_rounds(input: &str) -> IResult<&str, Vec<Round>> {
        separated_list1(tuple((tag(";"), space0)), parse_round)(input)
    }

    fn parse_game(input: &str) -> IResult<&str, Game> {
        map(tuple((tag("Game "), digit1, tag(": "), parse_rounds)), |(_, id, _, rounds)| {
            Game::new(id.parse().unwrap(), rounds)
        })(input)
    }

    pub(crate) fn parse(input: &str) -> Result<Vec<Game>, ParseError> {
        separated_list1(line_ending, parse_game)(input)
            .map_err(|e| ParseError::Nom(e.to_string()))
            .map(|(_, games)| games)
    }
}

#[cfg(test)]
mod tests {
    use super::models::*;
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_parse() {
        let solution = Day02 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!(
            vec![
                Game::new(
                    1,
                    vec![
                        Round::new(vec![Draw::from_enum(3, Colour::Blue), Draw::from_enum(4, Colour::Red)]),
                        Round::new(vec![
                            Draw::from_enum(1, Colour::Red),
                            Draw::from_enum(2, Colour::Green),
                            Draw::from_enum(6, Colour::Blue),
                        ]),
                        Round::new(vec![Draw::from_enum(2, Colour::Green)]),
                    ]
                ),
                Game::new(
                    2,
                    vec![
                        Round::new(vec![Draw::from_enum(1, Colour::Blue), Draw::from_enum(2, Colour::Green)]),
                        Round::new(vec![
                            Draw::from_enum(3, Colour::Green),
                            Draw::from_enum(4, Colour::Blue),
                            Draw::from_enum(1, Colour::Red),
                        ]),
                        Round::new(vec![Draw::from_enum(1, Colour::Green), Draw::from_enum(1, Colour::Blue)]),
                    ]
                ),
                Game::new(
                    3,
                    vec![
                        Round::new(vec![
                            Draw::from_enum(8, Colour::Green),
                            Draw::from_enum(6, Colour::Blue),
                            Draw::from_enum(20, Colour::Red),
                        ]),
                        Round::new(vec![
                            Draw::from_enum(5, Colour::Blue),
                            Draw::from_enum(4, Colour::Red),
                            Draw::from_enum(13, Colour::Green),
                        ]),
                        Round::new(vec![Draw::from_enum(5, Colour::Green), Draw::from_enum(1, Colour::Red)]),
                    ]
                ),
                Game::new(
                    4,
                    vec![
                        Round::new(vec![
                            Draw::from_enum(1, Colour::Green),
                            Draw::from_enum(3, Colour::Red),
                            Draw::from_enum(6, Colour::Blue),
                        ]),
                        Round::new(vec![Draw::from_enum(3, Colour::Green), Draw::from_enum(6, Colour::Red)]),
                        Round::new(vec![
                            Draw::from_enum(3, Colour::Green),
                            Draw::from_enum(15, Colour::Blue),
                            Draw::from_enum(14, Colour::Red),
                        ]),
                    ]
                ),
                Game::new(
                    5,
                    vec![
                        Round::new(vec![
                            Draw::from_enum(6, Colour::Red),
                            Draw::from_enum(1, Colour::Blue),
                            Draw::from_enum(3, Colour::Green),
                        ]),
                        Round::new(vec![
                            Draw::from_enum(2, Colour::Blue),
                            Draw::from_enum(1, Colour::Red),
                            Draw::from_enum(2, Colour::Green),
                        ]),
                    ]
                ),
            ],
            parsed
        );
    }

    #[test]
    fn test_part_one() {
        let solution = Day02 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!("8", solution.part1(parsed).unwrap().to_string());
    }

    #[test]
    fn test_part_two() {
        let solution = Day02 {};
        let parsed = solution.parse(INPUT).unwrap();
        assert_eq!("2286", solution.part2(parsed).unwrap().to_string());
    }

    #[test]
    fn test_minimum_counts() {
        let game = Game::new(
            1,
            vec![
                Round::new(vec![Draw::from_enum(3, Colour::Blue), Draw::from_enum(4, Colour::Red)]),
                Round::new(vec![
                    Draw::from_enum(1, Colour::Red),
                    Draw::from_enum(2, Colour::Green),
                    Draw::from_enum(6, Colour::Blue),
                ]),
                Round::new(vec![Draw::from_enum(2, Colour::Green)]),
            ],
        );
        assert_eq!(Counts { red: 4, green: 2, blue: 6 }, game.get_minimum_counts(),);
    }
}
