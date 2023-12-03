use std::cmp::max;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Colour {
    Red,
    Blue,
    Green,
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

#[derive(Debug, Clone)]
pub(crate) struct Draw {
    colour: Colour,
    amount: u16,
}
impl Draw {
    pub fn from_enum(amount: u16, colour: Colour) -> Self {
        Self { amount, colour }
    }
    pub fn from_str(amount: u16, colour: &str) -> Self {
        Self::from_enum(amount, Colour::from_str(colour))
    }
}

#[derive(Debug, PartialEq, Default)]
pub(crate) struct Counts {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

#[derive(Debug, Clone)]
pub(crate) struct Round {
    draws: Vec<Draw>,
}
impl Round {
    pub fn new(draws: Vec<Draw>) -> Self {
        Self { draws }
    }

    pub fn count_for(&self, colour: Colour) -> u16 {
        self.draws
            .iter()
            .filter(|draw| draw.colour == colour)
            .map(|draw| draw.amount)
            .sum()
    }

    pub fn get_counts(&self) -> Counts {
        Counts {
            red: self.count_for(Colour::Red),
            green: self.count_for(Colour::Green),
            blue: self.count_for(Colour::Blue),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Game {
    id: u16,
    rounds: Vec<Round>,
}
impl Game {
    pub fn new(id: u16, rounds: Vec<Round>) -> Self {
        Self { id, rounds }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }

    pub fn get_rounds(&self) -> Vec<Round> {
        self.rounds.clone()
    }

    pub fn get_minimum_counts(&self) -> Counts {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_counts() {
        let game = Game {
            id: 1,
            rounds: vec![
                Round {
                    draws: vec![
                        Draw {
                            amount: 3,
                            colour: Colour::Blue,
                        },
                        Draw {
                            amount: 4,
                            colour: Colour::Red,
                        },
                    ],
                },
                Round {
                    draws: vec![
                        Draw {
                            amount: 1,
                            colour: Colour::Red,
                        },
                        Draw {
                            amount: 2,
                            colour: Colour::Green,
                        },
                        Draw {
                            amount: 6,
                            colour: Colour::Blue,
                        },
                    ],
                },
                Round {
                    draws: vec![Draw {
                        amount: 2,
                        colour: Colour::Green,
                    }],
                },
            ],
        };

        assert_eq!(
            Counts {
                red: 4,
                green: 2,
                blue: 6
            },
            game.get_minimum_counts(),
        );
    }
}
