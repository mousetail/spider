use serde::{Deserialize, Serialize};
use crate::action::GameState;
use crate::cheats::Cheat::RedealStacks;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Cheat {
    RedealStacks(Vec<u8>),
    HarvestTopRow { turn_over_cards: [bool; 10]}
}

pub static CHEAT_NAMES: &[&str] = &[
    "re-deal stacks",
    "Harvest top row",
];

pub fn generate_cheat(game_state: &GameState, cheat_number: usize) -> Option<Cheat> {
    match cheat_number {
        0 => todo!(),
        1 => {
            Some(Cheat::HarvestTopRow{
                turn_over_cards: game_state.stacks.clone().map(
                    |i| i.len() >=2 && !i[i.len()-2].face_up
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
                    stack.last_mut().unwrap().face_up = true;
                }
            });
        },
        Cheat::RedealStacks(stacks) => {
            todo!()
        }
    }
}

pub fn undo_cheat(game_state: &mut GameState, cheat: Cheat) {
    match cheat {
        Cheat::HarvestTopRow { turn_over_cards } => {
            game_state.stacks.iter_mut().zip(turn_over_cards).for_each(|(stack, turn_over) | {
                if turn_over {
                    stack.last_mut().unwrap().face_up = false;
                }
            });

            let cards = game_state.deck.split_off(game_state.deck.len() - 10);
            for (stack, mut card) in game_state.stacks.iter_mut().zip(cards.into_iter()) {
                card.face_up = true;
                stack.push(card);
            }


        },
        Cheat::RedealStacks(stacks) => {
            todo!()
        }
    }
}