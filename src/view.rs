use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::vec::IntoIter;

use crate::terminal::rustbox::RustBoxTerminal;
use crate::terminal::{Style, Terminal};

const STYLE_NORMAL: Style = rustbox::RB_NORMAL;

pub enum Area
{
    LineNumber,
    Buffer,
    Status,
}

pub struct View
{
    terminal: Arc<Terminal + Sync + Send>,
}

impl View
{
    pub fn new() -> Self
    {
        let view = Self {
            terminal: Arc::new(RustBoxTerminal::new()),
        };

        view
    }

    pub fn terminal(&self) -> Arc<Terminal + Sync + Send>
    {
        Arc::clone(&self.terminal)
    }

    pub fn size(&mut self) -> (usize, usize)
    {
        self.terminal.size()
    }

    pub fn present(&mut self)
    {
        self.terminal.present();
    }

    pub fn set_cursor(&mut self, x: isize, y: isize)
    {
        self.terminal.set_cursor(x, y);
    }

    pub fn render_status(&mut self, cursor: (isize, isize), row: isize, status_text: &str)
    {
        let (cx, cy) = cursor;
        let status_color = (rustbox::Color::Black, rustbox::Color::Green);
        self.terminal
            .print((0, row), STYLE_NORMAL, status_color, status_text.as_ref());
    }

    pub fn render_buffer(
        &mut self,
        lines: IntoIter<(usize, Option<&str>)>,
        area: (usize, usize, usize, usize),
    )
    {
        let color = (rustbox::Color::White, rustbox::Color::Black);
        for (i, line) in lines {
            if line.is_none() {
                break;
            }
            self.terminal.print(
                (area.0 as isize, i as isize),
                STYLE_NORMAL,
                color,
                line.unwrap(),
            );
            // line number
            self.terminal.print(
                (0, i as isize),
                STYLE_NORMAL,
                color,
                format!(" {}", i).as_ref(),
            );
        }
    }
}
