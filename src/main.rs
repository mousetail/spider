use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::spawn;
use signal_hook::consts::{SIGABRT, SIGHUP, SIGINT, SIGQUIT, SIGTERM, SIGTSTP};
use signal_hook::iterator::Signals;
use crate::cards::GameState;
use crate::tui::{Input, draw, get_input};

mod cards;
mod tui;

enum InputState {
    SelectSource,
    SelectDestination(usize),
}

fn run_game(running: &AtomicBool) {

    let _terminal = tui::init().unwrap();
    let mut game_state = GameState::init(&mut rand::rng());
    let mut input_state = InputState::SelectSource;

    while running.load(Ordering::Relaxed) {
        draw(&game_state).unwrap();

        let value = get_input().unwrap();
        match value {
            Input::Undo => {}
            Input::Deal => {}
            Input::Row(e) => {
                let e = e as usize;

                match input_state {
                    InputState::SelectSource => {
                        input_state = InputState::SelectDestination(e);
                    }
                    InputState::SelectDestination(source) => {
                        game_state.move_from_to(source, e);

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


    let mut signals = Signals::new([
        SIGINT,
        SIGABRT,
        SIGTERM,
        SIGQUIT,
        SIGHUP,
        SIGTSTP
    ]).unwrap();
    let sig_handle = signals.handle();

    thread::scope(|scope| {
        eprintln!("Starting thread");
        let handle = scope.spawn(|| {
            for sig in signals.forever() {
                eprintln!("Got signal {sig}");
                match sig {
                    SIGINT | SIGTERM | SIGQUIT | SIGHUP  | SIGABRT=> {
                        keep_running.store(false, Ordering::Relaxed);

                        eprintln!("Exiting thead...");
                        exit(0);
                        break;
                    },
                    SIGTSTP => {

                    },
                    _ => unreachable!(),
                };
            }
        });

        run_game(&keep_running);
        sig_handle.close();

        handle.join().unwrap();
    });

    println!("Exited");
    exit(0);
    println!("Thanks for playing!");
}
