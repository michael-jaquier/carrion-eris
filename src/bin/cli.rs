//! Demonstrates how to match on modifiers like: Control, alt, shift.
//!
//! cargo run --example event-poll-read

use carrion_eris::{
    game::cli::game_loop::GameStates,
    ui::cli::{GameClient, RawMode, TICK_RATE},
};

use crossterm::{
    event::{poll, read, Event, KeyCode},
};

use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

fn main() -> io::Result<()> {
    let mut client = GameClient::new();
    let mut stdout = io::stdout();
    let _raw_mode = RawMode::enable()?;

    let mut game = GameStates::new();
    while !client.quit {
        while poll(Duration::ZERO)? {
            match read()? {
                Event::Resize(w, h) => {
                    client.resize(w, h);
                }
                Event::FocusGained => {}
                Event::FocusLost => {}
                Event::Key(event) => match event.code {
                    KeyCode::Backspace => {
                        client.prompt.backspace();
                    }
                    KeyCode::Enter => {
                        {
                            let command = client.prompt.buffer.iter().collect::<String>();
                            game.command(command, &mut client)
                        }
                        client.prompt.clear();
                    }

                    KeyCode::Left => {
                        client.prompt.left_word();
                    }
                    KeyCode::Right => {
                        client.prompt.right_word();
                    }
                    KeyCode::Up => {}
                    KeyCode::Down => {}
                    KeyCode::Home => {}
                    KeyCode::End => {}
                    KeyCode::PageUp => {}
                    KeyCode::PageDown => {}
                    KeyCode::Tab => {
                        client.quit = true;
                    }
                    KeyCode::BackTab => {}
                    KeyCode::Delete => {}
                    KeyCode::Insert => {}
                    KeyCode::F(_) => {}
                    KeyCode::Char(c) => {
                        client.prompt.insert(c);
                    }
                    KeyCode::Null => {}
                    KeyCode::Esc => {}
                    KeyCode::CapsLock => {}
                    KeyCode::ScrollLock => {}
                    KeyCode::NumLock => {}
                    KeyCode::PrintScreen => {}
                    KeyCode::Pause => {}
                    KeyCode::Menu => {}
                    KeyCode::KeypadBegin => {}
                    KeyCode::Media(_) => {}
                    KeyCode::Modifier(_) => {}
                },
                Event::Mouse(_) => {}
                Event::Paste(_) => {}
            }
        }

        game.update(&mut client);
        client.render_prompt()?;
        stdout.flush()?;

        thread::sleep(Duration::from_millis(TICK_RATE));
    }
    Ok(())
}
