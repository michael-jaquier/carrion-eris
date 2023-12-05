//! Demonstrates how to match on modifiers like: Control, alt, shift.
//!
//! cargo run --example event-poll-read

use carrion_eris::game::gamesync::{GameState, GameStates, GameSync};
use carrion_eris::{class::Classes, game_loop::battle_info};
use carrion_eris::{game, ValidEnum};
use crossterm::{
    cursor::MoveTo,
    event::{poll, read, Event, KeyCode},
    style::{
        Color, Print, PrintStyledContent, ResetColor, SetBackgroundColor, SetForegroundColor,
        Stylize,
    },
    terminal::{self, Clear},
    QueueableCommand,
};
use std::process::exit;
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};
use tracing::error;

const HELP: &str = r#"Blocking poll() & non-blocking read()
 - Keyboard, mouse and terminal resize events enabled
 - Prints "." every second if there's no event
 - Hit "c" to print current cursor position
 - Use Esc to quit
"#;

#[derive(Debug, Default)]
pub struct Client {
    quit: bool,
    messages: Messages,
}
fn status_bar(
    qc: &mut impl QueueableCommand,
    label: &str,
    x: usize,
    y: usize,
    w: usize,
) -> io::Result<()> {
    if label.len() <= w {
        qc.queue(MoveTo(x as u16, y as u16))?;
        qc.queue(SetBackgroundColor(Color::White))?;
        qc.queue(SetForegroundColor(Color::Black))?;
        qc.queue(Print(label))?;
        for _ in 0..w as usize - label.len() {
            qc.queue(Print(" "))?;
        }
        qc.queue(ResetColor)?;
    }
    Ok(())
}
struct Rect {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

#[derive(Debug, Default)]
struct Messages {
    items: Vec<(String, Color)>,
}

impl Messages {
    fn push(&mut self, message: String, color: Color) {
        self.items.push((message, color));
    }
    fn render(&mut self, qc: &mut impl QueueableCommand, boundary: Rect) -> io::Result<()> {
        let n = self.items.len();
        let m = n.checked_sub(boundary.h).unwrap_or(0);
        for (dy, (line, color)) in self.items.iter().skip(m).enumerate() {
            qc.queue(MoveTo(boundary.x as u16, (boundary.y + dy) as u16))?;
            qc.queue(PrintStyledContent(
                line.get(0..boundary.w).unwrap_or(&line).with(*color),
            ))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
struct Prompt {
    buffer: Vec<char>,
    cursor: usize,
}

impl Prompt {
    fn insert(&mut self, x: char) {
        if self.cursor > self.buffer.len() {
            self.cursor = self.buffer.len()
        }
        self.buffer.insert(self.cursor, x);
        self.cursor += 1;
    }

    fn insert_str(&mut self, text: &str) {
        for x in text.chars() {
            self.insert(x)
        }
    }

    fn left_char(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    fn right_char(&mut self) {
        if self.cursor < self.buffer.len() {
            self.cursor += 1;
        }
    }

    fn at_cursor(&self) -> char {
        self.buffer.get(self.cursor).cloned().unwrap_or('\n')
    }

    fn left_word(&mut self) {
        while self.cursor > 0 && self.at_cursor().is_whitespace() {
            self.cursor -= 1;
        }
        while self.cursor > 0 && !self.at_cursor().is_whitespace() {
            self.cursor -= 1;
        }
    }

    fn right_word(&mut self) {
        while self.cursor < self.buffer.len() && self.at_cursor().is_whitespace() {
            self.cursor += 1;
        }
        while self.cursor < self.buffer.len() && !self.at_cursor().is_whitespace() {
            self.cursor += 1;
        }
    }

    fn backspace(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.buffer.remove(self.cursor);
        }
    }

    fn before_cursor(&self) -> &[char] {
        &self.buffer[..self.cursor]
    }

    fn after_cursor(&self) -> &[char] {
        &self.buffer[self.cursor..]
    }

    fn clear(&mut self) {
        self.buffer.clear();
        self.cursor = 0;
    }
}

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
struct RawMode;

impl RawMode {
    fn enable() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        Ok(RawMode)
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        let _ =
            terminal::disable_raw_mode().map_err(|err| eprintln!("ERROR: disable raw mode: {err}"));
    }
}

fn main() -> io::Result<()> {
    let mut client = Client::default();
    let mut stdout = io::stdout();
    let _raw_mode = RawMode::enable()?;
    let mut prompt = Prompt::default();
    let (mut w, mut h) = terminal::size()?;

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();

    let first_msg = format!("Choose a class");
    let binding = Classes::valid();
    let valid_classes = binding.split('\n').collect::<Vec<_>>();

    chat_msg!(&mut client.messages, "{}", first_msg);
    for x in valid_classes.iter() {
        chat_msg!(&mut client.messages, "\t{}", x);
    }
    let mut game_state: GameState = GameState::new();
    let mut previous_prompt = String::new();
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
                            let prompt = prompt.buffer.iter().collect::<String>();
                            chat_msg!(&mut client.messages, "{text}", text = &prompt);
                            previous_prompt = prompt;
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
