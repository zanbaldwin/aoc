mod display;
pub mod part1;
pub mod part2;

#[derive(Debug)]
pub enum Error {
    NotYetImplemented,
    Other(String),
    InvalidCardCharacter(char),
    WrongNumberOfCards(usize),
}
impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Other(value.to_string())
    }
}

mod models {
    use super::Error;
    use std::{cmp::Ordering, collections::HashMap};

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

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub(crate) enum JokeredCard {
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
    impl TryFrom<char> for JokeredCard {
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
    impl From<Card> for JokeredCard {
        fn from(card: Card) -> Self {
            match card {
                Card::Two => Self::Two,
                Card::Three => Self::Three,
                Card::Four => Self::Four,
                Card::Five => Self::Five,
                Card::Six => Self::Six,
                Card::Seven => Self::Seven,
                Card::Eight => Self::Eight,
                Card::Nine => Self::Nine,
                Card::Ten => Self::Ten,
                Card::Jack => Self::Jack,
                Card::Queen => Self::Queen,
                Card::King => Self::King,
                Card::Ace => Self::Ace,
            }
        }
    }
    impl From<JokeredCard> for Card {
        fn from(card: JokeredCard) -> Self {
            match card {
                JokeredCard::Two => Self::Two,
                JokeredCard::Three => Self::Three,
                JokeredCard::Four => Self::Four,
                JokeredCard::Five => Self::Five,
                JokeredCard::Six => Self::Six,
                JokeredCard::Seven => Self::Seven,
                JokeredCard::Eight => Self::Eight,
                JokeredCard::Nine => Self::Nine,
                JokeredCard::Ten => Self::Ten,
                JokeredCard::Jack => Self::Jack,
                JokeredCard::Queen => Self::Queen,
                JokeredCard::King => Self::King,
                JokeredCard::Ace => Self::Ace,
            }
        }
    }

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
    impl TryFrom<Vec<JokeredCard>> for HandType {
        type Error = Error;
        fn try_from(cards: Vec<JokeredCard>) -> Result<Self, Self::Error> {
            if !cards.contains(&JokeredCard::Jack) {
                // If the hand doesn't contain any jokers then at least get a
                // little win wherever possible because part 2 is beginning to
                // hurt my brain.
                let cards: Vec<Card> = cards.into_iter().map(|card| card.into()).collect();
                return HandType::try_from(cards);
            }

            let joker_count = cards
                .iter()
                .filter(|card| *card == &JokeredCard::Jack)
                .count();
            if joker_count == 5 {
                // No further processing required.
                return Ok(HandType::FiveOfAKind);
            }

            let remaining_cards: Vec<JokeredCard> = cards
                .iter()
                .filter_map(|card| {
                    if *card != JokeredCard::Jack {
                        Some(*card)
                    } else {
                        None
                    }
                })
                .collect();

            let mut map: HashMap<JokeredCard, usize> = HashMap::new();
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

            let cards: Vec<Card> = cards
                .into_iter()
                .map(|card| -> Card {
                    if card == JokeredCard::Jack {
                        max_card.into()
                    } else {
                        card.into()
                    }
                })
                .collect();

            HandType::try_from(cards)
        }
    }

    pub(crate) struct ParsedHand {
        pub(crate) cards: Vec<Card>,
        pub(crate) bid: usize,
    }
    impl ParsedHand {
        pub fn new(hand: &str, bid: usize) -> Result<Self, Error> {
            let cards: Result<Vec<Card>, Error> = hand.chars().map(|c| c.try_into()).collect();
            let cards = cards?;
            if cards.len() != 5 {
                return Err(Error::WrongNumberOfCards(cards.len()));
            }
            Ok(Self { cards, bid })
        }
    }

    pub(crate) trait Hand: PartialEq + Eq + PartialOrd + Ord + TryFrom<ParsedHand> {
        fn score(&self, rank: usize) -> usize;
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub(crate) struct HandWithoutJokers {
        pub(crate) cards: Vec<Card>,
        pub(crate) hand_type: HandType,
        pub(crate) bid: usize,
    }
    impl TryFrom<ParsedHand> for HandWithoutJokers {
        type Error = Error;
        fn try_from(hand: ParsedHand) -> Result<Self, Self::Error> {
            let hand_type: HandType = hand.cards.clone().try_into()?;
            Ok(HandWithoutJokers {
                cards: hand.cards,
                hand_type,
                bid: hand.bid,
            })
        }
    }
    impl Hand for HandWithoutJokers {
        fn score(&self, rank: usize) -> usize {
            self.bid * rank
        }
    }
    // For some reason, the derive macro versions of PartialOrd cause the
    // ordering to fail. Will look into that another time, I guess.
    impl PartialOrd for HandWithoutJokers {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for HandWithoutJokers {
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

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub(crate) struct HandWithJokers {
        pub(crate) cards: Vec<JokeredCard>,
        pub(crate) hand_type: HandType,
        pub(crate) bid: usize,
    }
    impl TryFrom<ParsedHand> for HandWithJokers {
        type Error = Error;
        fn try_from(hand: ParsedHand) -> Result<Self, Self::Error> {
            let jokered_cards: Vec<JokeredCard> =
                hand.cards.into_iter().map(|card| card.into()).collect();
            let hand_type: HandType = jokered_cards.clone().try_into()?;
            Ok(HandWithJokers {
                cards: jokered_cards,
                hand_type,
                bid: hand.bid,
            })
        }
    }
    impl Hand for HandWithJokers {
        fn score(&self, rank: usize) -> usize {
            self.bid * rank
        }
    }
    // For some reason, the derive macro versions of PartialOrd cause the
    // ordering to fail. Will look into that another time, I guess.
    impl PartialOrd for HandWithJokers {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for HandWithJokers {
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
}

mod parser {
    use crate::Error;

    use super::models::*;
    use nom::{
        bytes::complete::take,
        character::complete::{digit1, line_ending, space1},
        combinator::{map, verify},
        multi::separated_list1,
        sequence::separated_pair,
        IResult,
    };

    const VALID_CARD_CHARACTERS: &str = "23456789TJQKA";

    pub(crate) fn parse(input: &str) -> miette::Result<Vec<ParsedHand>, Error> {
        common::nom(parse_playlist, input)
    }

    fn parse_playlist(input: &str) -> IResult<&str, Vec<ParsedHand>> {
        separated_list1(line_ending, parse_hand)(input)
    }

    fn parse_hand(input: &str) -> IResult<&str, ParsedHand> {
        map(
            separated_pair(
                verify(take(5usize), |cards: &str| {
                    cards.chars().all(|c| VALID_CARD_CHARACTERS.contains(c))
                }),
                space1,
                digit1,
            ),
            |(cards, bid)| ParsedHand::new(cards, bid.parse().unwrap()).unwrap(),
        )(input)
    }
}
