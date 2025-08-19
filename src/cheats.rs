use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};
use crate::action::GameState;
use crate::cards::{Card, Suit};
use crate::SpiderRand;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Cheat {
    RedealStacks{ cards: Vec<u8>, suit: Suit, prev_rng_state: Box<SpiderRand>, next_rng_state: Box<SpiderRand> },
    HarvestTopRow { turn_over_cards: [bool; 10]}
}

pub static CHEAT_NAMES: &[&str] = &[
    "re-deal stacks",
    "Harvest top row",
];

pub fn generate_cheat(game_state: &GameState, cheat_number: usize) -> Option<Cheat> {
    match cheat_number {
        0 => {
            let suit = *game_state.completed_stacks.last()?;

            let mut rng = game_state.rng.clone();

            let mut cards = (0..13).collect::<Vec<_>>();
            cards.shuffle(&mut rng);

            Some(Cheat::RedealStacks{ cards, suit, prev_rng_state: Box::new(game_state.rng.clone()), next_rng_state: Box::new(rng) })
        },
        1 => {
            Some(Cheat::HarvestTopRow{
                turn_over_cards: game_state.stacks.clone().map(
                    |i| i.len() >=2 && !i[i.len()-2].is_facing_up
                )
            })
        },
        _ => None
    }
}

pub fn apply_cheat(game_state: &mut GameState, cheat: Cheat) {
    match cheat {
        Cheat::HarvestTopRow {turn_over_cards} => {
            // Exactly the same as undoing deal
            let vc: Vec<_> = game_state.stacks.iter_mut().map(|d| d.pop().unwrap()).collect();

            game_state.deck.extend(vc);


            game_state.stacks.iter_mut().zip(turn_over_cards).for_each(|(stack, turn_over) | {
                if turn_over {
                    stack.last_mut().unwrap().is_facing_up = true;
                }
            });
        },
        Cheat::RedealStacks{ cards: ranks, suit, prev_rng_state: _, next_rng_state } => {
            game_state.rng = *next_rng_state;
            assert_eq!(game_state.completed_stacks.pop(), Some(suit));

            let mut cards = ranks.iter().map(
                |e| Card {
                    suit,
                    rank: *e,
                    is_facing_up: true,
                }
            );

            game_state.deck.extend((&mut cards).take(ranks.len() - ranks.len() % 10));

            cards.zip(game_state.stacks.iter_mut()).for_each(|(card, stack)| {
                stack.push(card);
            });
        }
    }
}

pub fn undo_cheat(game_state: &mut GameState, cheat: Cheat) {
    match cheat {
        Cheat::HarvestTopRow { turn_over_cards } => {
            game_state.stacks.iter_mut().zip(turn_over_cards).for_each(|(stack, turn_over) | {
                if turn_over {
                    stack.last_mut().unwrap().is_facing_up = false;
                }
            });

            let cards = game_state.deck.split_off(game_state.deck.len() - 10);
            for (stack, mut card) in game_state.stacks.iter_mut().zip(cards.into_iter()) {
                card.is_facing_up = true;
                stack.push(card);
            }


        },
        Cheat::RedealStacks{ cards, suit, prev_rng_state, next_rng_state: _ } => {
            game_state.rng = *prev_rng_state;

            let remainder = cards.len() % 10;
            (0..remainder).zip(game_state.stacks.iter_mut()).for_each(|(_card, stack)| {
                stack.pop().unwrap();
            });
            game_state.deck.truncate(game_state.deck.len() - 10);
            game_state.completed_stacks.push(suit);
        }
    }
}