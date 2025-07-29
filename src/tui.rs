use crossterm::{terminal, ExecutableCommand};
use std::io;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};

pub struct Terminal;

impl Drop for Terminal {
    fn drop(&mut self) {
        io::stdout().execute(terminal::LeaveAlternateScreen).unwrap();
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
    Quit
}

pub fn get_input() -> Result<Input, io::Error> {
    loop {
        let ev = read()?;
        match ev {
            Event::Key(KeyEvent { code: KeyCode::Esc | KeyCode::Char('q'), .. }) => return Ok(Input::Quit),
            Event::Key(KeyEvent { code: KeyCode::Char(char @ '1'..='9'), modifiers, .. }) => {
                let value = char.to_digit(10).unwrap();
                return Ok(Input::Row(value))
            }
            _ => ()
        }
    }
}