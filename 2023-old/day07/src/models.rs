use super::Error;
use std::fmt;

pub(crate) mod joker;
pub(crate) mod nojoker;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
impl fmt::Display for HandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let ht = match self {
            Self::HighCard => "High-card",
            Self::OnePair => "One-pair",
            Self::TwoPair => "Two-pair",
            Self::ThreeOfAKind => "Three-of-a-kind",
            Self::FullHouse => "Full-house",
            Self::FourOfAKind => "Four-of-a-kind",
            Self::FiveOfAKind => "Five-of-a-kind",
        };
        write!(f, "{ht}")
    }
}

pub(crate) struct ParsedHand {
    pub(crate) cards: Vec<nojoker::Card>,
    pub(crate) bid: usize,
}
impl ParsedHand {
    pub fn new(hand: &str, bid: usize) -> Result<Self, Error> {
        let cards: Result<Vec<nojoker::Card>, Error> = hand.chars().map(|c| c.try_into()).collect();
        let cards = cards?;
        if cards.len() != 5 {
            return Err(Error::WrongNumberOfCards(cards.len()));
        }
        Ok(Self { cards, bid })
    }
}

pub(crate) trait HandTrait: PartialEq + Eq + PartialOrd + Ord + TryFrom<ParsedHand> {
    fn score(&self, rank: usize) -> usize;
}
