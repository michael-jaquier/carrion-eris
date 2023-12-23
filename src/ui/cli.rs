use std::{fmt::Debug, io};

use crossterm::{
    cursor::MoveTo,
    style::{
        Color, Print, PrintStyledContent, ResetColor, SetBackgroundColor, SetForegroundColor,
        Stylize,
    },
    terminal, QueueableCommand,
};

pub fn status_bar(
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

pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

#[derive(Debug, Default)]
pub struct Prompt {
    pub buffer: Vec<char>,
    pub cursor: usize,
}

impl Prompt {
    pub fn insert(&mut self, x: char) {
        if self.cursor > self.buffer.len() {
            self.cursor = self.buffer.len()
        }
        self.buffer.insert(self.cursor, x);
        self.cursor += 1;
    }

    pub fn insert_str(&mut self, text: &str) {
        for x in text.chars() {
            self.insert(x)
        }
    }

    pub fn left_char(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn right_char(&mut self) {
        if self.cursor < self.buffer.len() {
            self.cursor += 1;
        }
    }

    pub fn at_cursor(&self) -> char {
        self.buffer.get(self.cursor).cloned().unwrap_or('\n')
    }

    pub fn left_word(&mut self) {
        while self.cursor > 0 && self.at_cursor().is_whitespace() {
            self.cursor -= 1;
        }
        while self.cursor > 0 && !self.at_cursor().is_whitespace() {
            self.cursor -= 1;
        }
    }

    pub fn right_word(&mut self) {
        while self.cursor < self.buffer.len() && self.at_cursor().is_whitespace() {
            self.cursor += 1;
        }
        while self.cursor < self.buffer.len() && !self.at_cursor().is_whitespace() {
            self.cursor += 1;
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.buffer.remove(self.cursor);
        }
    }

    pub fn before_cursor(&self) -> &[char] {
        &self.buffer[..self.cursor]
    }

    pub fn after_cursor(&self) -> &[char] {
        &self.buffer[self.cursor..]
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.cursor = 0;
    }
}

pub struct RawMode;

impl RawMode {
    pub fn enable() -> io::Result<Self> {
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

type TerminalMessage = (String, Color);
type TerminalMessages = Vec<TerminalMessage>;

#[derive(Debug, Default)]
struct Messages {
    items: TerminalMessages,
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
pub struct GameClient {
    pub quit: bool,
    messages: Messages,
}

impl GameClient {
    pub fn send(&mut self, msg: TerminalMessages) {
        for ms in msg.into_iter() {
            self.messages.push(ms.0, ms.1);
        }
    }
    pub fn render(&mut self, qc: &mut impl QueueableCommand, boundary: Rect) -> io::Result<()> {
        self.messages.render(qc, boundary)
    }
}
