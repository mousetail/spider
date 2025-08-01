use std::fmt::Display;
use std::iter::Rev;
use std::ops::RangeInclusive;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum Suit {
    Clubs,
    Hearts,
    Diamonds,
    Spades,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CardColor {
    Red,
    Black,
}

impl Suit {
    fn get_color(&self) -> CardColor {
        match self {
            Suit::Clubs => CardColor::Black,
            Suit::Hearts => CardColor::Red,
            Suit::Diamonds => CardColor::Red,
            Suit::Spades => CardColor::Black,
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: u8,
    pub face_up: bool,
}

impl Card {
    pub(crate) fn get_color(&self) -> CardColor {
        self.suit.get_color()
    }
}

#[derive(Clone)]
pub struct CardRange {
    pub suit: Suit,
    pub rank: Rev<RangeInclusive<u8>>,
    pub face_up: bool,
}

impl CardRange {
    pub fn len(&self) -> usize {
        self.rank.len()
    }

    pub fn contains_rank(&self, rank: u8) -> bool {
        let (last, first) = (self.rank.clone().next(), self.rank.clone().last());

        first
            .zip(last)
            .is_some_and(|(first, last)| first <= rank && rank <= last)
    }
}
impl Iterator for CardRange {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        self.rank.next().map(|e| Card {
            suit: self.suit,
            rank: e,
            face_up: self.face_up,
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
            if first.face_up && card.face_up && card.suit == last.suit && card.rank + 1 == last.rank {
                last = card;
                last_index = inex;
            } else {
                break;
            }
        }

        self.0 = &self.0[last_index + 1..];

        Some(CardRange {
            suit: first.suit,
            face_up: first.face_up,
            rank: (last.rank..=first.rank).rev(),
        })
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.face_up {
            write!(
                f,
                "{}{}",
                match self.rank {
                    0 => 'A',
                    x @ 1..9 => (x + '1' as u8) as char,
                    9 => 'X',
                    10 => 'J',
                    11 => 'Q',
                    12 => 'K',
                    x => panic!("Expected rank 0-13 exclusive, got: {x}"),
                },
                self.suit
            )
        } else {
            write!(f, "██")
        }
    }
}
