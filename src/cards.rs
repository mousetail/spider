use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Display;
use std::iter::Rev;
use std::ops::RangeInclusive;

const FACE_UP_CHAR: char = '↑';
const FACE_DOWN_CHAR: char = '↓';

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) enum Suit {
    #[serde(rename = "♣")]
    Clubs,
    #[serde(rename = "♥")]
    Hearts,
    #[serde(rename = "♦")]
    Diamonds,
    #[serde(rename = "♠")]
    Spades,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CardColor {
    Red,
    Black,
}

impl Suit {
    pub fn get_color(&self) -> CardColor {
        match self {
            Suit::Hearts | Suit::Diamonds => CardColor::Red,
            Suit::Clubs | Suit::Spades => CardColor::Black,
        }
    }

    pub fn from_char(v: char) -> Option<Self> {
        match v {
            '♣' => Some(Suit::Clubs),
            '♥' => Some(Suit::Hearts),
            '♦' => Some(Suit::Diamonds),
            '♠' => Some(Suit::Spades),
            _ => None,
        }
    }
}
impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Suit::Clubs => "♣",
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Spades => "♠",
        })
    }
}

impl Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        format!(
            "{}{}{}",
            self.suit,
            self.get_rank_char(),
            if self.is_facing_up {
                FACE_UP_CHAR
            } else {
                FACE_DOWN_CHAR
            }
        )
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let [suit, rank, is_facing_up] = String::deserialize(deserializer)?
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .map_err(|_| D::Error::custom("Expected 3 length string"))?;

        Ok(Card {
            suit: Suit::from_char(suit).ok_or_else(|| D::Error::custom("Unexpected suit"))?,
            rank: Card::get_rank_from_char(rank)
                .ok_or_else(|| D::Error::custom("Unexpected rank"))?,
            is_facing_up: match is_facing_up {
                FACE_UP_CHAR => true,
                FACE_DOWN_CHAR => false,
                _ => Err(D::Error::custom("Unexpected is_facing_up"))?,
            },
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: u8,
    pub is_facing_up: bool,
}

impl Card {
    fn get_rank_char(&self) -> char {
        Self::rank_to_char(self.rank)
    }

    fn rank_to_char(rank: u8) -> char {
        match rank {
            0 => 'A',
            x @ 1..9 => (x + b'1') as char,
            9 => 'X',
            10 => 'J',
            11 => 'Q',
            12 => 'K',
            x => panic!("Expected rank 0-13 exclusive, got: {x}"),
        }
    }

    fn get_rank_from_char(c: char) -> Option<u8> {
        match c {
            'A' => Some(0),
            '2'..='9' => Some(c as u8 - b'1'),
            'X' => Some(9),
            'J' => Some(10),
            'Q' => Some(11),
            'K' => Some(12),
            _ => None,
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_facing_up {
            write!(f, "{}{}", self.get_rank_char(), self.suit)
        } else {
            write!(f, "██")
        }
    }
}

#[derive(Clone)]
pub struct CardRange {
    pub suit: Suit,
    pub rank: Rev<RangeInclusive<u8>>,
    pub is_facing_up: bool,
}

impl Serialize for CardRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        format!(
            "{}{}-{}{}",
            self.suit,
            Card::rank_to_char(self.rank.clone().next().unwrap()),
            Card::rank_to_char(self.rank.clone().next_back().unwrap()),
            if self.is_facing_up {
                FACE_UP_CHAR
            } else {
                FACE_DOWN_CHAR
            }
        )
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CardRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let [suit, from, _, to, is_facing_up] = String::deserialize(deserializer)?
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .map_err(|_| D::Error::custom("Expected 5 length string"))?;

        Ok(CardRange {
            suit: Suit::from_char(suit).ok_or_else(|| Error::custom("Bad suit"))?,
            rank: (Card::get_rank_from_char(to).ok_or_else(|| Error::custom("Bad to rank"))?
                ..=Card::get_rank_from_char(from).ok_or_else(|| Error::custom("Bad from rank"))?)
                .rev(),
            is_facing_up: match is_facing_up {
                FACE_UP_CHAR => true,
                FACE_DOWN_CHAR => false,
                _ => Err(D::Error::custom("Unexpected is_facing_up"))?,
            },
        })
    }
}

impl CardRange {
    pub fn len(&self) -> usize {
        self.rank.len()
    }

    pub fn contains_rank(&self, rank: u8) -> bool {
        let (last, first) = (self.first(), self.clone().last());

        first
            .zip(last)
            .is_some_and(|(first, last)| first.rank <= rank && rank <= last.rank)
    }

    pub fn first(&self) -> Option<Card> {
        Some(Card {
            rank: self.rank.clone().next()?,
            suit: self.suit,
            is_facing_up: self.is_facing_up,
        })
    }
}
impl Iterator for CardRange {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        self.rank.next().map(|e| Card {
            suit: self.suit,
            rank: e,
            is_facing_up: self.is_facing_up,
        })
    }

    fn last(self) -> Option<Self::Item> {
        Some(Card {
            rank: self.rank.clone().next()? + 1 - self.rank.len() as u8,
            suit: self.suit,
            is_facing_up: self.is_facing_up,
        })
    }
}

pub struct Groups<'a>(pub &'a [Card]);

impl<'a> Iterator for Groups<'a> {
    type Item = CardRange;
    fn next(&mut self) -> Option<Self::Item> {
        let first = *self.0.first()?;
        let mut last = first;
        let mut last_index = 0;
        for (inex, &card) in self.0.iter().enumerate().skip(1) {
            if first.is_facing_up
                && card.is_facing_up
                && card.suit == last.suit
                && card.rank + 1 == last.rank
            {
                last = card;
                last_index = inex;
            } else {
                break;
            }
        }

        self.0 = &self.0[last_index + 1..];

        Some(CardRange {
            suit: first.suit,
            is_facing_up: first.is_facing_up,
            rank: (last.rank..=first.rank).rev(),
        })
    }
}
