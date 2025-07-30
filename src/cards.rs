use rand::prelude::SliceRandom;
use std::fmt::Display;
use std::iter::Rev;
use std::ops::RangeInclusive;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Suit {
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

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Card {
    suit: Suit,
    rank: u8,
    pub face_up: bool,
}

impl Card {
    pub(crate) fn get_color(&self) -> CardColor {
        self.suit.get_color()
    }
}

#[derive(Clone)]
pub struct CardRange {
    suit: Suit,
    rank: Rev<RangeInclusive<u8>>,
    face_up: bool,
}

impl CardRange {
    fn len(&self) -> usize {self.rank.len()}
}
impl Iterator for CardRange {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        self.rank.next().map(|e| Card {
            suit: self.suit,
            rank: e,
            face_up: self.face_up
        })
    }
}

pub struct Groups<'a>(&'a [Card]);

impl<'a> Iterator for Groups<'a> {
    type Item = CardRange;
    fn next(&mut self) -> Option<Self::Item> {
        let first = *self.0.first()?;
        let mut last = first;
        let mut last_index = 0;
        for (inex, &card) in self.0.iter().enumerate().skip(1) {
            if card.face_up && card.suit == last.suit && card.rank + 1 == last.rank {
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
                    x @ 0..9 => (x + '1' as u8) as char,
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

pub struct GameState {
    pub stacks: [Vec<Card>; 10],
    pub deck: Vec<Card>,
}

impl GameState {
    pub fn init(rand: &mut impl rand::Rng) -> GameState {
        let mut all_cards: Vec<_> = (0..13)
            .flat_map(|e| {
                [
                    Card {
                        suit: Suit::Clubs,
                        rank: e as u8,
                        face_up: false,
                    },
                    Card {
                        suit: Suit::Hearts,
                        rank: e as u8,
                        face_up: false,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: e as u8,
                        face_up: false,
                    },
                    Card {
                        suit: Suit::Hearts,
                        rank: e as u8,
                        face_up: false,
                    },
                ]
                .repeat(2)
            })
            .collect();

        all_cards.shuffle(rand);

        let mut state = GameState {
            stacks: [
                (&all_cards[50..56]).to_vec(), // 4 stacks of 5
                (&all_cards[56..62]).to_vec(),
                (&all_cards[62..68]).to_vec(),
                (&all_cards[68..74]).to_vec(),
                (&all_cards[74..79]).to_vec(), // 6 stacks of 4
                (&all_cards[79..84]).to_vec(),
                (&all_cards[84..89]).to_vec(),
                (&all_cards[89..94]).to_vec(),
                (&all_cards[94..99]).to_vec(),
                (&all_cards[99..104]).to_vec(),
            ],
            deck: (&all_cards[..50]).to_vec(),
        };

        for row in &mut state.stacks {
            row.last_mut().unwrap().face_up = true;
        }

        state
    }

    pub fn move_from_to(&mut self, from: usize, to: usize) -> Option<()> {
        let last_group = Groups(&self.stacks[from]).last()?;

        self.stacks[from].truncate(self.stacks[from].len() - last_group.len());
        self.stacks[to].extend(last_group);

        if let Some(e) = self.stacks[from].last_mut() {
            e.face_up = true;
        }

        Some(())
    }
}
