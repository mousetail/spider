use crate::action::{Action, GameState};
use crate::tui::{Input, draw, get_input};
use crossterm::{ExecutableCommand, terminal};
use signal_hook::consts::{SIGABRT, SIGHUP, SIGINT, SIGQUIT, SIGTERM, SIGTSTP};
use signal_hook::iterator::Signals;
use std::io::stdout;
use std::panic::{set_hook, take_hook};
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{io, thread};

mod action;
mod cards;
mod tui;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum InputState {
    SelectSource,
    SelectDestination(usize),
}

fn run_game(running: &AtomicBool) {
    let _terminal = tui::init().unwrap();
    let mut undo_stack: Vec<Action> = Vec::new();
    let mut game_state = GameState::init(&mut rand::rng());
    let mut input_state = InputState::SelectSource;
    let mut changed= true;

    while running.load(Ordering::Relaxed) {
        if changed {
            draw(&game_state, input_state).unwrap();
            changed = false;
        }

        let value = get_input().unwrap();
        match value {
            Input::Undo => {}
            Input::Deal => {}
            Input::Row(e) => {
                let e = e as usize;

                match input_state {
                    InputState::SelectSource => {
                        input_state = InputState::SelectDestination(e);
                        changed = true;
                    }
                    InputState::SelectDestination(source) => {
                        let action = game_state.move_from_to(source, e);
                        if let Some(action) = action {
                            println!("Action");
                            undo_stack.push(action.clone());
                            game_state.apply_action(action);
                            changed = true;
                        } else {
                            println!("\rNo action available");
                        }

                        input_state = InputState::SelectSource;
                    }
                }
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
