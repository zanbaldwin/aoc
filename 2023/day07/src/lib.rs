mod display;
pub mod part1;
pub mod part2;

mod models {
    use std::{cmp::Ordering, collections::HashMap};

    #[derive(Debug)]
    pub(crate) enum Error {
        InvalidCardCharacter(char),
        WrongNumberOfCards(usize),
    }

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

    #[derive(Debug, PartialEq, Eq, PartialOrd)]
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
        fn order(&self) -> u8 {
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
    impl TryFrom<Vec<Card>> for HandType {
        type Error = Error;
        fn try_from(cards: Vec<Card>) -> Result<Self, Error> {
            if cards.len() != 5 {
                return Err(Error::WrongNumberOfCards(cards.len()));
            }

            let mut map: HashMap<Card, u8> = HashMap::new();
            for card in cards {
                *map.entry(card).or_insert(0) += 1;
            }

            map.try_into()
        }
    }
    impl TryFrom<HashMap<Card, u8>> for HandType {
        type Error = Error;
        fn try_from(map: HashMap<Card, u8>) -> Result<Self, Self::Error> {
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

    #[derive(Debug, PartialEq, Eq)]
    pub(crate) struct Hand {
        pub(crate) cards: Vec<Card>,
        pub(crate) hand_type: HandType,
        pub(crate) bid: u32,
    }
    impl Hand {
        pub fn new(hand: &str, bid: u32) -> Result<Self, Error> {
            let cards: Result<Vec<Card>, Error> = hand.chars().map(|c| c.try_into()).collect();
            let cards = cards?;
            let hand_type: HandType = cards.clone().try_into()?;
            Ok(Self {
                cards,
                hand_type,
                bid,
            })
        }

        pub fn score(&self, rank: usize) -> u64 {
            (self.bid as u64) * (rank as u64)
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
    // For some reason, the derive macro versions of PartialOrd cause the
    // ordering to fail. Will look into that another time, I guess.
    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
}

mod parser {
    use super::models::*;
    use common::AocError;
    use nom::{
        bytes::complete::take,
        character::complete::{digit1, line_ending, space1},
        combinator::{map, verify},
        multi::separated_list1,
        sequence::separated_pair,
        IResult,
    };

    const VALID_CARD_CHARACTERS: &str = "23456789TJQKA";

    pub(crate) fn parse(input: &str) -> miette::Result<Vec<Hand>, AocError> {
        common::nom(parse_playlist, input)
    }

    fn parse_playlist(input: &str) -> IResult<&str, Vec<Hand>> {
        separated_list1(line_ending, parse_hand)(input)
    }

    fn parse_hand(input: &str) -> IResult<&str, Hand> {
        map(
            separated_pair(
                verify(take(5usize), |cards: &str| {
                    cards.chars().all(|c| VALID_CARD_CHARACTERS.contains(c))
                }),
                space1,
                digit1,
            ),
            |(cards, bid)| Hand::new(cards, bid.parse().unwrap()).unwrap(),
        )(input)
    }
}
