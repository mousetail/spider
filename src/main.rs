use crossterm::event::{read, Event, KeyCode};
use crossterm::event::KeyEvent as Key;
use crate::tui::{get_input, Input};

mod tui;
mod cards;

fn main() {
    println!("Hello, world!");

    let _terminal = tui::init().unwrap();

    loop {
        let value = get_input().unwrap();
        match value {
            Input::Undo => {}
            Input::Deal => {}
            Input::Row(_) => {}
            Input::Quit => break
        }
    }
}
