use std::sync::Arc;
use std::vec::IntoIter;

use crate::terminal::rustbox::RustBoxTerminal;
use crate::terminal::{Style, Terminal};

const STYLE_NORMAL: Style = rustbox::RB_NORMAL;

pub struct View
{
    terminal: Arc<Terminal + Sync + Send>,
}

impl View
{
    pub fn new() -> Self
    {
        Self {
            terminal: Arc::new(RustBoxTerminal::new()),
        }
    }

    pub fn terminal(&self) -> Arc<Terminal + Sync + Send>
    {
        Arc::clone(&self.terminal)
    }

    pub fn size(&mut self) -> (usize, usize)
    {
        self.terminal.size()
    }

    pub fn clear(&mut self)
    {
        self.terminal.clear();
    }

    pub fn present(&mut self)
    {
        self.terminal.present();
    }

    pub fn set_cursor(&mut self, x: i64, y: i64)
    {
        self.terminal.set_cursor(x, y);
    }

    pub fn render_status(&mut self, _cursor: (i64, i64), row: i64, status_text: &str)
    {
        let status_color = (rustbox::Color::Black, rustbox::Color::Red);
        self.terminal
            .print((0, row), STYLE_NORMAL, status_color, &status_text);

        {
            let status_len = status_text.len();
            let padding_len = self.terminal.size().0 - status_len;

            let padding_text = (0..padding_len).map(|_| " ").collect::<String>();
            self.terminal.print(
                (status_len as i64, row),
                STYLE_NORMAL,
                status_color,
                &padding_text,
            );
        }
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
                (area.0 as i64, i as i64),
                STYLE_NORMAL,
                color,
                line.unwrap(),
            );
            // line number
            self.terminal.print(
                (0, i as i64),
                STYLE_NORMAL,
                color,
                format!(" {}", i).as_ref(),
            );
        }
    }
}
