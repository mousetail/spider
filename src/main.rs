use crate::action::{Action, GameState};
use crate::cards::Groups;
use crate::cheats::generate_cheat;
use crate::tui::{Input, draw, get_input};
use crossterm::{ExecutableCommand, terminal};
use rand_xoshiro::Xoshiro512StarStar;
use serde::{Deserialize, Serialize};
use signal_hook::consts::{SIGABRT, SIGHUP, SIGINT, SIGQUIT, SIGTERM, SIGTSTP};
use signal_hook::iterator::Signals;
use std::fs::{OpenOptions, exists};
use std::io::stdout;
use std::panic::{set_hook, take_hook};
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

mod action;
mod cards;
mod cheats;
mod tui;
mod help;

pub type SpiderRand = Xoshiro512StarStar;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum InputState {
    SelectSource,
    SelectDestination(usize),
    CheatMenu,
}

#[derive(Clone, Serialize, Deserialize)]
struct StateWithUndoHistory {
    state: GameState,
    undo_stack: Vec<Action>,
}

impl StateWithUndoHistory {
    fn save(&self) {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("spider-save.json")
            .unwrap();

        serde_json::to_writer_pretty(file, self).unwrap();
    }
    fn perform_action(&mut self, action: Action) {
        self.state.apply_action(action.clone());
        self.undo_stack.push(action);

        self.save();

        let mut actions = vec![];
        for (index, stack) in self.state.stacks.iter().enumerate() {
            if let Some(g) = Groups(stack)
                .last()
                .and_then(|e| (e.len() == 13).then_some(e))
            {
                actions.push((
                    g.suit,
                    index,
                    stack.len() > 13 && !stack[stack.len() - 14].face_up,
                ));
            }
        }
        for (suit, stack, flip_card) in actions {
            self.perform_action(Action::RemoveFullStack {
                suit,
                stack,
                flip_card,
            })
        }
    }

    fn undo(&mut self) {
        let action = if let Some(action) = self.undo_stack.pop() {
            action
        } else {
            return;
        };

        self.state.undo_action(action);
    }
}

fn run_game(running: &AtomicBool) {
    let _terminal = tui::init().unwrap();

    let mut game_state = if exists("spider-save.json").unwrap() {
        serde_json::from_reader(
            OpenOptions::new()
                .read(true)
                .open("spider-save.json")
                .unwrap(),
        )
        .unwrap()
    } else {
        StateWithUndoHistory {
            state: GameState::init(&mut rand::rng()),
            undo_stack: Vec::new(),
        }
    };
    let mut input_state = InputState::SelectSource;
    let mut changed = true;

    while running.load(Ordering::Relaxed) {
        if changed {
            draw(&game_state.state, input_state).unwrap();
            changed = false;
        }

        let value = get_input().unwrap();
        match value {
            Input::Restart => {
                if std::fs::exists("spider-save.backup.json").unwrap() {
                    std::fs::remove_file("spider-save.backup.json").unwrap();
                }
                std::fs::rename("spider-save.json", "spider-save.backup.json").unwrap();

                game_state = StateWithUndoHistory {
                    state: GameState::init(&mut rand::rng()),
                    undo_stack: Vec::new(),
                };

                changed=true;
            }
            Input::Undo => {
                game_state.undo();
                game_state.save();

                changed = true;
            }
            Input::Deal => {
                if game_state.state.deck.len() >= 10 {
                    game_state.perform_action(Action::Deal);

                    changed = true;
                }
            }
            Input::Row(e) => {
                let e = e as usize;

                match input_state {
                    InputState::SelectSource => {
                        input_state = InputState::SelectDestination(e);
                        changed = true;
                    }
                    InputState::SelectDestination(source) => {
                        let action = game_state.state.move_from_to(source, e);
                        if let Some(action) = action {
                            println!("Action");
                            game_state.perform_action(action);
                        }
                        changed = true;

                        input_state = InputState::SelectSource;
                    }
                    InputState::CheatMenu => {
                        input_state = InputState::SelectSource;

                        if let Some(cheat) = generate_cheat(&game_state.state, e) {
                            game_state.perform_action(Action::Cheat(cheat));
                            changed = true;
                        }
                    }
                }
            }
            Input::ShowCheatMenu => {
                input_state = InputState::CheatMenu;
                changed = true;
            }
            Input::Quit => break,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let keep_running = AtomicBool::new(true);

    let default_hook = take_hook();
    set_hook(Box::new(move |info| {
        stdout().execute(terminal::LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();

        default_hook(info);
    }));

    let mut signals = Signals::new([SIGINT, SIGABRT, SIGTERM, SIGQUIT, SIGHUP, SIGTSTP]).unwrap();
    let sig_handle = signals.handle();

    thread::scope(|scope| {
        eprintln!("Starting thread");
        let handle = scope.spawn(|| {
            for sig in signals.forever() {
                eprintln!("Got signal {sig}");
                match sig {
                    SIGINT | SIGTERM | SIGQUIT | SIGHUP | SIGABRT => {
                        keep_running.store(false, Ordering::Relaxed);

                        eprintln!("Exiting thead...");
                        exit(0);
                    }
                    SIGTSTP => {}
                    _ => unreachable!(),
                };
            }
        });

        run_game(&keep_running);
        sig_handle.close();

        handle.join().unwrap();
    });

    println!("Thanks for playing");
    exit(0);
}
