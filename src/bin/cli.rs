//! Demonstrates how to match on modifiers like: Control, alt, shift.
//!
//! cargo run --example event-poll-read

use carrion_eris::class::Classes;
use carrion_eris::game::gamesync::Event as GameEvent;
use carrion_eris::game::gamesync::{Context, GameState};
use carrion_eris::ui::cli::{status_bar, GameClient, Prompt, RawMode, Rect};
use carrion_eris::ValidEnum;
use carrion_patterns::fsm::StateMachine;
use crossterm::{
    cursor::MoveTo,
    event::{poll, read, Event, KeyCode},
    style::{Color, Print},
    terminal::{self, Clear},
    QueueableCommand,
};

use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

macro_rules! chat_msg {
    ($chat:expr, $($arg:tt)*) => {
        $chat.push(format!($($arg)*), Color::White)
    }
}

macro_rules! game_msg_helper {
    ($chat:expr, $($arg:literal)*) => {
        $(
            for line in $arg.split('\n') {
                chat_msg!($chat, "{}", line);
            }
        )*
    };
}

macro_rules! game_msg {
    ($chat:expr, $arg:expr) => {
        for line in format!("{}", $arg).split('\n') {
            chat_msg!($chat, "{}", line);
        }
    };
}

fn main() -> io::Result<()> {
    let mut client = GameClient::default();
    let mut stdout = io::stdout();
    let _raw_mode = RawMode::enable()?;
    let mut prompt = Prompt::default();
    let (mut w, mut h) = terminal::size()?;

    while !client.quit {
        while poll(Duration::ZERO)? {
            match read()? {
                Event::Resize(nw, nh) => {
                    w = nw;
                    h = nh;
                }
                Event::FocusGained => {}
                Event::FocusLost => {}
                Event::Key(event) => match event.code {
                    KeyCode::Backspace => {
                        prompt.backspace();
                    }
                    KeyCode::Enter => {
                        {
                            let _prompt = prompt.buffer.iter().collect::<String>();
                        }
                        prompt.clear();
                    }

                    KeyCode::Left => {
                        prompt.left_word();
                    }
                    KeyCode::Right => {
                        prompt.right_word();
                    }
                    KeyCode::Up => todo!(),
                    KeyCode::Down => todo!(),
                    KeyCode::Home => todo!(),
                    KeyCode::End => todo!(),
                    KeyCode::PageUp => todo!(),
                    KeyCode::PageDown => todo!(),
                    KeyCode::Tab => todo!(),
                    KeyCode::BackTab => todo!(),
                    KeyCode::Delete => todo!(),
                    KeyCode::Insert => todo!(),
                    KeyCode::F(_) => todo!(),
                    KeyCode::Char(c) => {
                        prompt.insert(c);
                    }
                    KeyCode::Null => todo!(),
                    KeyCode::Esc => todo!(),
                    KeyCode::CapsLock => todo!(),
                    KeyCode::ScrollLock => todo!(),
                    KeyCode::NumLock => todo!(),
                    KeyCode::PrintScreen => todo!(),
                    KeyCode::Pause => todo!(),
                    KeyCode::Menu => todo!(),
                    KeyCode::KeypadBegin => todo!(),
                    KeyCode::Media(_) => todo!(),
                    KeyCode::Modifier(_) => todo!(),
                },
                Event::Mouse(_) => {}
                Event::Paste(_) => {}
            }
        }

        stdout.queue(Clear(terminal::ClearType::All))?;
        stdout.queue(MoveTo(0, 0))?;
        status_bar(&mut stdout, "4at", 0, 0, w.into())?;

        client.messages.render(
            &mut stdout,
            Rect {
                x: 0,
                y: 1,
                w: w as usize,
                h: h as usize - 3,
            },
        )?;
        status_bar(&mut stdout, "Status: Online", 0, h as usize - 2, w.into())?;
        stdout.queue(MoveTo(0, h as u16 - 1))?;
        for x in prompt
            .buffer
            .get(0..(w - 2) as usize)
            .unwrap_or(&prompt.buffer)
        {
            stdout.queue(Print(x))?;
        }
        stdout.queue(MoveTo(prompt.cursor as u16, h - 1))?;
        stdout.flush()?;

        thread::sleep(Duration::from_millis(33));
    }
    Ok(())
}
