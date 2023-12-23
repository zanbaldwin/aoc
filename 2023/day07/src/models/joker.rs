use super::{HandTrait, HandType, ParsedHand};
use crate::{models::nojoker::Card as NoJokerCard, Error};
use std::{cmp::Ordering, collections::HashMap, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Card {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
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
impl From<NoJokerCard> for Card {
    fn from(card: NoJokerCard) -> Self {
        match card {
            NoJokerCard::Two => Self::Two,
            NoJokerCard::Three => Self::Three,
            NoJokerCard::Four => Self::Four,
            NoJokerCard::Five => Self::Five,
            NoJokerCard::Six => Self::Six,
            NoJokerCard::Seven => Self::Seven,
            NoJokerCard::Eight => Self::Eight,
            NoJokerCard::Nine => Self::Nine,
            NoJokerCard::Ten => Self::Ten,
            NoJokerCard::Jack => Self::Jack,
            NoJokerCard::Queen => Self::Queen,
            NoJokerCard::King => Self::King,
            NoJokerCard::Ace => Self::Ace,
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
        if !cards.contains(&Card::Jack) {
            // If the hand doesn't contain any jokers then at least get a
            // little win wherever possible because part 2 is beginning to
            // hurt my brain.
            let cards: Vec<NoJokerCard> = cards.into_iter().map(|card| card.into()).collect();
            return HandType::try_from(cards);
        }

        let joker_count = cards.iter().filter(|card| *card == &Card::Jack).count();
        if joker_count == 5 {
            // No further processing required.
            return Ok(HandType::FiveOfAKind);
        }

        let remaining_cards: Vec<Card> =
            cards.iter().filter_map(|card| if *card != Card::Jack { Some(*card) } else { None }).collect();

        let mut map: HashMap<Card, usize> = HashMap::new();
        for remaining_card in remaining_cards {
            *map.entry(remaining_card).or_insert(0) += 1;
        }
        let max_card = map
            .iter()
            .max_by(|(a_card, a_count), (b_card, b_count)| {
                let count_ord = a_count.cmp(b_count);
                if count_ord != Ordering::Equal {
                    return count_ord;
                }

                a_card.cmp(b_card)
            })
            .map(|(card, _count)| *card)
            .expect("there to always be another card if less than 5 jokers");

        let cards: Vec<NoJokerCard> = cards
            .into_iter()
            .map(|card| -> NoJokerCard {
                if card == Card::Jack {
                    max_card.into()
                } else {
                    card.into()
                }
            })
            .collect();

        HandType::try_from(cards)
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
        let jokered_cards: Vec<Card> = hand.cards.into_iter().map(|card| card.into()).collect();
        let hand_type: HandType = jokered_cards.clone().try_into()?;
        Ok(Self {
            cards: jokered_cards,
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
        let cards = self.cards.iter().map(|card| card.to_string()).collect::<Vec<_>>().join("");
        write!(f, "Bidding {:width$} for hand ({}): {}", self.bid, cards, self.hand_type, width = 4,)
    }
}
