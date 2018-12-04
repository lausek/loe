use std::time::Duration;

use rustbox::Key as RustBoxKey;
use rustbox::{Color, OutputMode, RustBox, Style};

use crate::input::{Event, Key};
use crate::terminal::{Position, Terminal};

pub type RustBoxColor = Color;
pub type RustBoxStyle = Style;

pub struct RustBoxTerminal
{
    rustbox: RustBox,
    timeout: Duration,
}

impl RustBoxTerminal
{
    pub fn new() -> Self
    {
        Self {
            timeout: Duration::from_millis(100),
            rustbox: {
                if let Ok(mut rb) = RustBox::init(Default::default()) {
                    rb.set_output_mode(OutputMode::EightBit);
                    rb
                } else {
                    panic!("could not initialize rustbox");
                }
            },
        }
    }
}

impl Terminal for RustBoxTerminal
{
    fn set_cursor(&self, x: i64, y: i64)
    {
        self.rustbox.set_cursor(x as isize, y as isize);
    }

    fn size(&self) -> (usize, usize)
    {
        (self.rustbox.width(), self.rustbox.height())
    }

    fn listen(&self) -> Option<Event>
    {
        match self.rustbox.peek_event(self.timeout, false) {
            Ok(rustbox::Event::KeyEvent(key)) => match key {
                RustBoxKey::Tab => Some(Event::Key(Key::Tab)),
                RustBoxKey::Enter => Some(Event::Key(Key::Enter)),
                RustBoxKey::Esc => Some(Event::Key(Key::Esc)),
                RustBoxKey::Backspace => Some(Event::Key(Key::Backspace)),
                RustBoxKey::Right => Some(Event::Key(Key::Right)),
                RustBoxKey::Left => Some(Event::Key(Key::Left)),
                RustBoxKey::Up => Some(Event::Key(Key::Up)),
                RustBoxKey::Down => Some(Event::Key(Key::Down)),
                RustBoxKey::Delete => Some(Event::Key(Key::Delete)),
                RustBoxKey::Insert => Some(Event::Key(Key::Insert)),
                RustBoxKey::Home => Some(Event::Key(Key::Home)),
                RustBoxKey::End => Some(Event::Key(Key::End)),
                RustBoxKey::PageUp => Some(Event::Key(Key::PageUp)),
                RustBoxKey::PageDown => Some(Event::Key(Key::PageDown)),
                RustBoxKey::Char(c) => Some(Event::Key(Key::Char(c))),
                RustBoxKey::Ctrl(c) => Some(Event::Key(Key::Ctrl(c))),
                _ => None,
            },
            Ok(rustbox::Event::ResizeEvent(_, _)) => Some(Event::Resize),
            _ => None,
        }
    }

    fn print(&self, position: Position, style: Style, color: crate::terminal::Color, content: &str)
    {
        let style = rustbox::Style::empty();
        let (fg, bg) = color;
        let (x, y) = position;
        self.rustbox
            .print(x as usize, y as usize, style, fg, bg, content);
    }

    fn present(&self)
    {
        self.rustbox.present();
    }

    fn clear(&self)
    {
        self.rustbox.clear();
    }
}
