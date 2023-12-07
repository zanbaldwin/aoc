use crate::models::*;
use std::fmt::Display;

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let card = match self {
            Self::Two => '2',
            Self::Three => '3',
            Self::Four => '4',
            Self::Five => '5',
            Self::Six => '6',
            Self::Seven => '7',
            Self::Eight => '8',
            Self::Nine => '9',
            Self::Ten => 'T',
            Self::Jack => 'J',
            Self::Queen => 'Q',
            Self::King => 'K',
            Self::Ace => 'A',
        };
        write!(f, "{card}")
    }
}

impl Display for HandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ht = match self {
            Self::HighCard(c) => format!("High-card ({c})"),
            Self::OnePair(c) => format!("One-pair ({c})"),
            Self::TwoPair(c1, c2) => format!("Two-pair ({c1}, {c2})"),
            Self::ThreeOfAKind(c) => format!("Three-of-a-kind ({c})"),
            Self::FullHouse(c1, c2) => format!("Full-house ({c1}, {c2})"),
            Self::FourOfAKind(c) => format!("Four-of-a-kind ({c})"),
            Self::FiveOfAKind(c) => format!("Five-of-a-kind ({c})"),
        };
        write!(f, "{ht}")
    }
}

impl Display for HandWithoutJokers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards = self
            .cards
            .iter()
            .map(|card| card.to_string())
            .collect::<Vec<_>>()
            .join("");
        write!(
            f,
            "Bidding {:width$} for hand ({}): {}",
            self.bid,
            cards,
            self.hand_type,
            width = 4,
        )
    }
}
impl Display for HandWithJokers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards = self
            .cards
            .iter()
            .map(|card| card.to_string())
            .collect::<Vec<_>>()
            .join("");
        write!(
            f,
            "Bidding {:width$} for hand ({}): {}",
            self.bid,
            cards,
            self.hand_type,
            width = 4,
        )
    }
}
