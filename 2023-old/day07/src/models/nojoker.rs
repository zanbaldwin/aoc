use super::{HandTrait, HandType, ParsedHand};
use crate::{models::joker::Card as JokerCard, Error};
use std::{cmp::Ordering, collections::HashMap, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
impl TryFrom<char> for Card {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            c => return Err(Error::InvalidCardCharacter(c)),
        })
    }
}
impl From<JokerCard> for Card {
    fn from(card: JokerCard) -> Self {
        match card {
            JokerCard::Two => Self::Two,
            JokerCard::Three => Self::Three,
            JokerCard::Four => Self::Four,
            JokerCard::Five => Self::Five,
            JokerCard::Six => Self::Six,
            JokerCard::Seven => Self::Seven,
            JokerCard::Eight => Self::Eight,
            JokerCard::Nine => Self::Nine,
            JokerCard::Ten => Self::Ten,
            JokerCard::Jack => Self::Jack,
            JokerCard::Queen => Self::Queen,
            JokerCard::King => Self::King,
            JokerCard::Ace => Self::Ace,
        }
    }
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
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

impl TryFrom<Vec<Card>> for HandType {
    type Error = Error;
    fn try_from(cards: Vec<Card>) -> Result<Self, Self::Error> {
        let mut map: HashMap<Card, usize> = HashMap::new();
        for card in cards {
            *map.entry(card).or_insert(0) += 1;
        }

        Ok(match map.len() {
            // FiveOfAKind (AAAAA).
            1 => Self::FiveOfAKind,
            // Either FourOfAKind (AAAAB) or FullHouse (AAABB).
            2 => {
                if map.iter().any(|(_card, count)| *count == 4) {
                    // FourOfAKind (AAAAB).
                    Self::FourOfAKind
                } else {
                    // FullHouse (AAABB).
                    Self::FullHouse
                }
            }
            // Either ThreeOfAKind(AAABC) or TwoPair (AABBC).
            3 => {
                if map.iter().any(|(_card, count)| *count == 3) {
                    Self::ThreeOfAKind
                } else {
                    Self::TwoPair
                }
            }
            // OnePair (AABCD).
            4 => Self::OnePair,
            // HighCard (ABCDE).
            5 => Self::HighCard,
            n => return Err(Error::WrongNumberOfCards(n)),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Hand {
    pub(crate) cards: Vec<Card>,
    pub(crate) hand_type: HandType,
    pub(crate) bid: usize,
}
impl TryFrom<ParsedHand> for Hand {
    type Error = Error;
    fn try_from(hand: ParsedHand) -> Result<Self, Self::Error> {
        let hand_type: HandType = hand.cards.clone().try_into()?;
        Ok(Self {
            cards: hand.cards,
            hand_type,
            bid: hand.bid,
        })
    }
}
impl HandTrait for Hand {
    fn score(&self, rank: usize) -> usize {
        self.bid * rank
    }
}
// For some reason, the derive macro versions of PartialOrd cause the
// ordering to fail. Will look into that another time, I guess.
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_order = self.hand_type.cmp(&other.hand_type);
        if hand_order != Ordering::Equal {
            return hand_order;
        }
        for (own_card, other_card) in self.cards.iter().zip(&other.cards) {
            let card_order = own_card.cmp(other_card);
            if card_order != Ordering::Equal {
                return card_order;
            }
        }
        Ordering::Equal
    }
}
impl fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
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
