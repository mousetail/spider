use crate::cards::{CardColor, GameState};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};
use crossterm::style::{Color, SetForegroundColor};
use crossterm::{ExecutableCommand, cursor, terminal};
use std::io;

pub struct Terminal;

impl Drop for Terminal {
    fn drop(&mut self) {
        io::stdout()
            .execute(terminal::LeaveAlternateScreen)
            .unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}
pub fn init() -> Result<Terminal, io::Error> {
    let mut stdout = io::stdout();

    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    terminal::enable_raw_mode()?;

    Ok(Terminal)
}

pub enum Input {
    Undo,
    Deal,
    Row(u32),
    Quit,
}

pub fn draw(game_state: &GameState) -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::MoveToRow(0))?;
    stdout.execute(cursor::MoveToColumn(0))?;

    for (index, row) in game_state.stacks.iter().enumerate() {
        stdout.execute(SetForegroundColor(Color::Grey))?;
        print!("{:>3}: ", (index + 1) % 10);
        for card in row {
            if !card.face_up {
                stdout.execute(SetForegroundColor(Color::Blue))?;
            }
            else if card.get_color() == CardColor::Red {
                stdout.execute(SetForegroundColor(Color::Red))?;
            } else {
                stdout.execute(SetForegroundColor(Color::White))?;
            }
            print!(" {} ", card);
        }
        println!("\r");
    }
    stdout.execute(SetForegroundColor(Color::Grey))?;

    Ok(())
}

pub fn get_input() -> Result<Input, io::Error> {
    loop {
        let ev = read()?;
        match ev {
            Event::Key(KeyEvent {
                code: KeyCode::Esc | KeyCode::Char('q' | 'c'),
                ..
            })  => return Ok(Input::Quit),
            Event::Key(KeyEvent {
                code: KeyCode::Char(char @ '1'..='9'),
                modifiers,
                ..
            }) => {
                let value = (char.to_digit(10).unwrap() + 9) % 10;
                return Ok(Input::Row(value));
            }
            _ => (),
        }
    }
}
