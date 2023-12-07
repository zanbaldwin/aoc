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

    #[derive(Debug, PartialEq, Eq)]
    pub(crate) enum HandType {
        HighCard(Card),
        OnePair(Card),
        TwoPair(Card, Card),
        ThreeOfAKind(Card),
        FullHouse(Card, Card),
        FourOfAKind(Card),
        FiveOfAKind(Card),
    }
    impl HandType {
        fn order(&self) -> usize {
            match self {
                Self::HighCard(_) => 1,
                Self::OnePair(_) => 2,
                Self::TwoPair(_, _) => 3,
                Self::ThreeOfAKind(_) => 4,
                Self::FullHouse(_, _) => 5,
                Self::FourOfAKind(_) => 6,
                Self::FiveOfAKind(_) => 7,
            }
        }
    }
    // We have to implement our own Ord trait for HandType because otherwise the
    // default derive macro version will also order by the values inside the
    // enum variants (and the puzzle specification explicitly tells us to ignore
    // that).
    impl Ord for HandType {
        fn cmp(&self, other: &Self) -> Ordering {
            self.order().cmp(&other.order())
        }
    }
    impl PartialOrd for HandType {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
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
                1 => {
                    // FiveOfAKind (AAAAA).
                    let any_card = map.keys().last().unwrap();
                    Self::FiveOfAKind(*any_card)
                }
                2 => {
                    // Either FourOfAKind (AAAAB) or FullHouse (AAABB).
                    if map.iter().any(|(_card, count)| *count == 4) {
                        // FourOfAKind (AAAAB).
                        let four_cards = map
                            .iter()
                            .filter(|(_card, count)| **count == 4)
                            .map(|(card, _count)| card)
                            .last()
                            .unwrap();
                        Self::FourOfAKind(*four_cards)
                    } else {
                        // FullHouse (AAABB).
                        let three_cards = map
                            .iter()
                            .filter(|(_card, count)| **count == 3)
                            .map(|(card, _count)| card)
                            .last()
                            .unwrap();
                        let two_cards = map
                            .iter()
                            .filter(|(_card, count)| **count == 2)
                            .map(|(card, _count)| card)
                            .last()
                            .unwrap();
                        Self::FullHouse(*three_cards, *two_cards)
                    }
                }
                3 => {
                    // Either ThreeOfAKind(AAABC) or TwoPair (AABBC).
                    if map.iter().any(|(_card, count)| *count == 3) {
                        let three_cards = map
                            .iter()
                            .filter(|(_card, count)| **count == 3)
                            .map(|(card, _count)| card)
                            .last()
                            .unwrap();
                        Self::ThreeOfAKind(*three_cards)
                    } else {
                        let mut two_cards: Vec<Card> = map
                            .into_iter()
                            .filter(|(_card, count)| *count == 2)
                            .map(|(card, _count)| card)
                            .collect();
                        two_cards.sort();
                        Self::TwoPair(two_cards[1], two_cards[0])
                    }
                }
                4 => {
                    // OnePair (AABCD).
                    let card = map
                        .into_iter()
                        .filter(|(_card, count)| *count == 2)
                        .map(|(card, _count)| card)
                        .last()
                        .unwrap();
                    Self::OnePair(card)
                }
                5 => {
                    // HighCard (ABCDE).
                    let high_card = map.keys().max().unwrap();
                    Self::HighCard(*high_card)
                }
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

            todo!()
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

    #[derive(Debug, PartialEq, Eq)]
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

    #[derive(Debug, PartialEq, Eq)]
    pub(crate) struct HandWithJokers {
        pub(crate) cards: Vec<Card>,
        pub(crate) hand_type: HandType,
        pub(crate) bid: usize,
    }
    impl TryFrom<ParsedHand> for HandWithJokers {
        type Error = Error;
        fn try_from(hand: ParsedHand) -> Result<Self, Self::Error> {
            let hand_type: HandType = hand
                .cards
                .clone()
                .into_iter()
                .map(|card| card.into())
                .collect::<Vec<JokeredCard>>()
                .try_into()?;
            Ok(HandWithJokers {
                cards: hand.cards,
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
