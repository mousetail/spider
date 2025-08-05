use crate::InputState;
use crate::action::GameState;
use crate::cards::{CardColor, Groups};
use crate::cheats::CHEAT_NAMES;
use crossterm::event::KeyCode::Modifier;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};
use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::{ExecutableCommand, cursor, terminal};
use std::io;
use std::io::Stdout;

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
    ShowCheatMenu,
    Restart,
}

fn draw_game(
    stdout: &mut Stdout,
    game_state: &GameState,
    source: Option<usize>,
) -> Result<(), io::Error> {
    stdout.execute(SetBackgroundColor(Color::Reset))?;
    for suit in game_state.completed_stacks.iter() {
        stdout.execute(SetForegroundColor(match suit.get_color() {
            CardColor::Red => Color::Red,
            CardColor::Black => Color::White,
        }))?;
        print!(" [{suit}] ")
    }
    println!("\r\n");

    for (index, row) in game_state.stacks.iter().enumerate() {
        let (bg, fg) = match source {
            None => (Color::Reset, Color::Reset),
            Some(e) => {
                if e == index {
                    (Color::White, Color::Black)
                } else if game_state.can_move_to(e, index).is_some() {
                    (Color::Green, Color::Reset)
                } else {
                    (Color::Reset, Color::Reset)
                }
            }
        };

        stdout.execute(SetBackgroundColor(bg))?;
        stdout.execute(SetForegroundColor(fg))?;

        print!("{:>3}: ", (index + 1) % 10);

        stdout.execute(ResetColor)?;

        for card in Groups(row) {
            if !card.face_up {
                stdout.execute(SetForegroundColor(Color::Blue))?;
            } else if card.suit.get_color() == CardColor::Red {
                stdout.execute(SetForegroundColor(Color::Red))?;
            } else {
                stdout.execute(SetForegroundColor(Color::White))?;
            }

            if card.len() <= 1 {
                print!(" {} ", card.clone().next().unwrap());
            } else {
                print!(
                    " {}{}{}",
                    card.first().unwrap(),
                    "-".repeat(card.len() - 1),
                    card.last().unwrap()
                );
            }
        }
        println!("\r");
        stdout.execute(SetForegroundColor(Color::Reset))?;
    }
    stdout.execute(SetForegroundColor(Color::Grey))?;
    println!("\r");

    for k in 0..game_state.deck.len() / 10 {
        print!("[{k:>3}] ");
    }

    println!("\r");

    Ok(())
}

fn draw_cheat_menu(stdout: &mut Stdout) -> Result<(), io::Error> {
    for (index, cheat) in CHEAT_NAMES.iter().enumerate() {
        stdout.execute(SetForegroundColor(Color::Grey))?;
        print!("{:>3}: ", index + 1);
        stdout.execute(SetForegroundColor(Color::Reset))?;
        println!("{cheat}\r");
    }

    Ok(())
}

pub fn draw(game_state: &GameState, input_state: InputState) -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::MoveToRow(0))?;
    stdout.execute(cursor::MoveToColumn(0))?;

    match input_state {
        InputState::SelectSource => {
            draw_game(&mut stdout, &game_state, None)?;
        }
        InputState::SelectDestination(v) => draw_game(&mut stdout, game_state, Some(v))?,
        InputState::CheatMenu => {
            draw_cheat_menu(&mut stdout)?;
        }
    }

    Ok(())
}

pub fn get_input() -> Result<Input, io::Error> {
    loop {
        let ev = read()?;
        match ev {
            Event::Key(KeyEvent {
                code: KeyCode::Char('C'),
                modifiers,
                ..
            }) if modifiers.contains(KeyModifiers::SHIFT)
                || modifiers.contains(KeyModifiers::CONTROL) =>
            {
                println!("Cheat menu opening");
                return Ok(Input::ShowCheatMenu);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Esc | KeyCode::Char('q' | 'c'),
                ..
            }) => return Ok(Input::Quit),
            Event::Key(KeyEvent {
                code: KeyCode::Char('R'),
                ..
            }) => return Ok(Input::Restart),

            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => return Ok(Input::Deal),

            Event::Key(KeyEvent {
                code: KeyCode::Char('u'),
                ..
            }) => return Ok(Input::Undo),
            Event::Key(KeyEvent {
                code: KeyCode::Char(char @ '0'..='9'),
                modifiers: _,
                ..
            }) => {
                let value = (char.to_digit(10).unwrap() + 9) % 10;
                return Ok(Input::Row(value));
            }
            _ => (),
        }
    }
}
