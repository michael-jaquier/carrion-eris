use std::{
    fmt::Debug,
    io::{self, Stdout},
};

use crossterm::{
    cursor::MoveTo,
    style::{
        Color, Print, PrintStyledContent, ResetColor, SetBackgroundColor, SetForegroundColor,
        Stylize,
    },
    terminal::{self, Clear},
    QueueableCommand,
};

pub static TICK_RATE: u64 = 3 * ((1.0 / 60.0) * 1000.0) as u64;

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
        for _ in 0..w - label.len() {
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

pub type TerminalMessage = (String, Color);
pub type TerminalMessages = Vec<TerminalMessage>;

pub type UpdateTerminalFn = fn(u64) -> (String, Color);

#[derive(Debug, Default)]
pub(crate) struct Messages {
    sequence: TerminalMessages,
    update: Option<(UpdateTerminalFn, u64)>,
}

impl From<TerminalMessage> for Messages {
    fn from(msg: TerminalMessage) -> Self {
        Self {
            sequence: vec![msg],
            update: None,
        }
    }
}

impl From<TerminalMessages> for Messages {
    fn from(msg: TerminalMessages) -> Self {
        Self {
            sequence: msg,
            update: None,
        }
    }
}

impl Messages {
    pub fn iter(&self) -> impl Iterator<Item = &TerminalMessage> {
        self.sequence.iter()
    }

    pub fn new() -> Self {
        Self {
            sequence: vec![],
            update: None,
        }
    }
    pub(crate) fn push(&mut self, message: String, color: Color) {
        self.sequence.push((message, color));
    }
    pub(crate) fn extend(&mut self, message: Vec<String>, color: Color) {
        for msg in message {
            self.sequence.push((msg, color));
        }
    }

    fn redraw(&self, qc: &mut impl QueueableCommand) -> io::Result<()> {
        qc.queue(Clear(terminal::ClearType::All))?;
        qc.queue(MoveTo(0, 0))?;
        Ok(())
    }

    fn render(&mut self, qc: &mut impl QueueableCommand, boundary: Rect) -> io::Result<()> {
        let n = self.sequence.len();
        let m = n.saturating_sub(boundary.h - 1);
        for (dy, (line, color)) in self.sequence.iter().skip(m).enumerate() {
            qc.queue(MoveTo(boundary.x as u16, (boundary.y + dy) as u16))?;
            qc.queue(PrintStyledContent(
                line.get(0..boundary.w).unwrap_or(line).with(*color),
            ))?;
        }

        // Draw the message at the last drawn line + 1
        if let Some((update_fn, update_param)) = self.update {
            let (line, color) = update_fn(update_param);
            qc.queue(MoveTo(boundary.x as u16, (boundary.y + n - m) as u16))?;
            qc.queue(PrintStyledContent(
                line.get(0..boundary.w).unwrap_or(&line).with(color),
            ))?;
        }
        Ok(())
    }

    pub fn send(&mut self, msg: Messages) {
        self.sequence.extend(msg.sequence);
        self.update = msg.update;
    }

    pub(crate) fn update(&mut self, terminal_function: UpdateTerminalFn, x: u64) {
        self.update = Some((terminal_function, x));
    }
}

#[derive(Debug)]
pub struct GameClient {
    pub quit: bool,
    messages: Messages,
    new_messages: bool,
    pub prompt: Prompt,
    io: Stdout,
    size: (u16, u16),
}

impl Default for GameClient {
    fn default() -> Self {
        Self::new()
    }
}

impl GameClient {
    pub fn new() -> Self {
        Self {
            quit: false,
            messages: Default::default(),
            new_messages: false,
            prompt: Default::default(),
            io: io::stdout(),
            size: terminal::size().unwrap(),
        }
    }

    pub fn send(&mut self, msg: TerminalMessages) {
        if !msg.is_empty() {
            self.new_messages = true
        }
        for ms in msg.into_iter() {
            self.messages.push(ms.0, ms.1);
        }
    }

    pub(crate) fn msg_send(&mut self, msg: Messages) {
        self.messages.send(msg);
    }

    pub fn render(&mut self, boundary: Option<Rect>) -> io::Result<()> {
        self.messages.redraw(&mut self.io)?;
        let (w, h) = self.size;
        let boundary = boundary.unwrap_or(Rect {
            x: 0,
            y: 1,
            w: w as usize,
            h: h as usize - 3,
        });
        if self.new_messages {
            self.end_of_turn(boundary.w);
            self.new_messages = false;
        }
        self.messages.render(&mut self.io, boundary)
    }

    pub fn status_bar_bottom(&mut self, message: &str) {
        let (w, h) = self.size;
        let _ = status_bar(&mut self.io, message, 0, h as usize - 2, w.into());
    }

    pub fn render_prompt(&mut self) -> io::Result<()> {
        self.io.queue(MoveTo(0, self.size.1 - 1))?;
        for x in self
            .prompt
            .buffer
            .get(0..(self.size.0 - 2) as usize)
            .unwrap_or(&self.prompt.buffer)
        {
            self.io.queue(Print(x))?;
        }
        self.io
            .queue(MoveTo(self.prompt.cursor as u16, self.size.1 - 1))?;

        Ok(())
    }

    fn end_of_turn(&mut self, w: usize) {
        self.send(vec![(" ".repeat(w), Color::White)]);
    }

    pub fn resize(&mut self, w: u16, h: u16) {
        self.size = (w, h)
    }
}
