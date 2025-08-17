use crate::cards::{Card, CardRange, Groups, Suit};
use crate::cheats::{apply_cheat, undo_cheat, Cheat};
use crate::SpiderRand;
use rand::prelude::SliceRandom;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Action {
    Move {
        range: CardRange,
        flip_card: bool,
        from: usize,
        to: usize,
    },
    Deal,
    RemoveFullStack {
        suit: Suit,
        stack: usize,
        #[serde(default)]
        flip_card: bool,
    },
    Cheat(Cheat),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GameState {
    pub stacks: [Vec<Card>; 10],
    pub deck: Vec<Card>,
    pub completed_stacks: Vec<Suit>,
    pub rng: SpiderRand,
}

impl GameState {
    pub fn init(rand: &mut impl rand::Rng) -> GameState {
        let mut all_cards: Vec<_> = (0..13)
            .flat_map(|e| {
                [
                    Card {
                        suit: crate::cards::Suit::Clubs,
                        rank: e as u8,
                        face_up: false,
                    },
                    Card {
                        suit: crate::cards::Suit::Hearts,
                        rank: e as u8,
                        face_up: false,
                    },
                    Card {
                        suit: crate::cards::Suit::Clubs,
                        rank: e as u8,
                        face_up: false,
                    },
                    Card {
                        suit: crate::cards::Suit::Hearts,
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
                all_cards[50..56].to_vec(), // 4 stacks of 5
                all_cards[56..62].to_vec(),
                all_cards[62..68].to_vec(),
                all_cards[68..74].to_vec(),
                all_cards[74..79].to_vec(), // 6 stacks of 4
                all_cards[79..84].to_vec(),
                all_cards[84..89].to_vec(),
                all_cards[89..94].to_vec(),
                all_cards[94..99].to_vec(),
                all_cards[99..104].to_vec(),
            ],
            deck: all_cards[..50].to_vec(),
            completed_stacks: vec![],
            rng: SpiderRand::from_os_rng(),
        };

        for row in &mut state.stacks {
            row.last_mut().unwrap().face_up = true;
        }

        state
    }

    pub fn can_move_to(&self, from: usize, to: usize) -> Option<CardRange> {
        if from == to {
            return None;
        }

        let last_group = Groups(&self.stacks[from]).last()?;
        let dest = self.stacks[to].last();
        match dest {
            Some(e) => e.rank.checked_sub(1).and_then(|wanted_rank| {
                last_group.contains_rank(wanted_rank).then(|| CardRange {
                    suit: last_group.suit,
                    face_up: last_group.face_up,
                    rank: (last_group.rank.clone().next_back().unwrap()..=e.rank - 1).rev(),
                })
            }),
            None => Some(last_group),
        }
    }

    pub fn move_from_to(&self, from: usize, to: usize) -> Option<Action> {
        let moved_cards = self.can_move_to(from, to)?;

        let set_face_up = self.stacks[from]
            .len()
            .checked_sub(moved_cards.len() + 1)
            .is_some_and(|i| !self.stacks[from][i].face_up);

        Some(Action::Move {
            range: moved_cards,
            flip_card: set_face_up,
            from,
            to,
        })
    }

    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::Move {
                from,
                to,
                flip_card,
                range,
            } => {
                self.stacks[from].truncate(self.stacks[from].len() - range.len());
                self.stacks[to].extend(range);

                if flip_card {
                    self.stacks[from].last_mut().unwrap().face_up = true;
                }
            }
            Action::Deal => {
                let cards = self.deck.split_off(self.deck.len() - 10);
                for (stack, mut card) in self.stacks.iter_mut().zip(cards.into_iter()) {
                    card.face_up = true;
                    stack.push(card);
                }
            }
            Action::RemoveFullStack {
                suit,
                stack,
                flip_card,
            } => {
                self.stacks[stack].truncate(self.stacks[stack].len() - 13);
                self.completed_stacks.push(suit);
                if flip_card {
                    self.stacks[stack].last_mut().unwrap().face_up = true;
                }
            }
            Action::Cheat(cheat) => apply_cheat(self, cheat),
        }
    }

    pub(crate) fn undo_action(&mut self, action: Action) {
        match action {
            Action::Move {
                from,
                to,
                flip_card,
                range,
            } => {
                if flip_card {
                    self.stacks[from].last_mut().unwrap().face_up = false;
                }

                self.stacks[to].truncate(self.stacks[to].len() - range.len());
                self.stacks[from].extend(range);
            }
            Action::Deal => {
                let vc: Vec<_> = self.stacks.iter_mut().map(|d| d.pop().unwrap()).collect();

                self.deck.extend(vc);
            }
            Action::RemoveFullStack {
                suit,
                stack,
                flip_card,
            } => {
                if flip_card {
                    self.stacks[stack].last_mut().unwrap().face_up = false;
                }
                self.completed_stacks.pop();
                self.stacks[stack].extend(CardRange {
                    suit,
                    rank: (0..=12).rev(),
                    face_up: true,
                });
            }
            Action::Cheat(cheat) => {
                undo_cheat(self, cheat);
            }
        }
    }
}
