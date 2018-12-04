use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::vec::IntoIter;

use crate::buffer::{Buffer, Buffers};
use crate::config::Config;
use crate::input::{Event, Key, Key::*};
use crate::mode::{Mode, Mode::*};
use crate::terminal::{Style, Terminal};
use crate::view::View;

pub struct App
{
    pub(crate) mode: Mode,
    pub(crate) command_buffer: String,
    config: Config,
    buffer: Option<Buffer>,
    current_buffer: Option<String>,
    margin: (i64, i64),
    events: Receiver<Event>,
    view: View,
}

impl App
{
    pub fn new(config: Config) -> Self
    {
        let (sender, receiver) = channel();

        let app = Self {
            config,
            buffer: None,
            current_buffer: None,
            command_buffer: String::new(),
            margin: (5, 0),
            events: receiver,
            mode: Mode::View,
            view: View::new(),
        };

        let mut r = app.view.terminal();
        thread::spawn(move || loop {
            if let Some(event) = r.listen() {
                sender.send(event).ok();
            }
        });

        app
    }

    pub fn with_args(mut self, args: std::env::Args) -> Self
    {
        if let Some(arg) = args.into_iter().skip(1).next() {
            self.buffer = Buffer::load(arg.as_ref()).ok();
        }
        self
    }

    pub fn run(&mut self) -> Result<(), &'static str>
    {
        loop {
            self.render();
            self.wait_for_event()?;

            if let Mode::Exit = self.mode {
                return Ok(());
            }
        }
    }

    pub fn render(&mut self)
    {
        self.view.clear();
        if let Some(buffer) = &mut self.buffer {
            let color = (rustbox::Color::White, rustbox::Color::Black);
            let (w, h) = {
                let size = self.view.size();
                (size.0, size.1 - 1)
            };

            {
                let area = (self.margin.0 as usize, 0usize, w, h);
                let lines_range = (0..h)
                    .map(|i| (i, buffer.get_row_at(i)))
                    .collect::<Vec<_>>()
                    .into_iter();
                self.view.render_buffer(lines_range, area);
            }

            let cursor_pos = buffer.get_cursor();
            self.view
                .set_cursor(self.margin.0 + cursor_pos.0, cursor_pos.1);

            let status_text = match &self.mode {
                Command => format!(
                    ":{} >> {}c {}r",
                    self.command_buffer, cursor_pos.0, cursor_pos.1
                ),
                _ => format!("{} >> {}c {}r", self.mode, cursor_pos.0, cursor_pos.1),
            };
            self.view
                .render_status(cursor_pos, h as i64, status_text.as_str());
        } else {
            log!("couldn't acquire current_buffer");
        }
        self.view.present();
    }

    pub fn wait_for_event(&mut self) -> Result<(), &'static str>
    {
        match self.events.recv() {
            Ok(event) => {
                match event {
                    Event::Resize => self.render(),
                    Event::Key(Up) => {
                        if let Some(buffer) = &mut self.buffer {
                            buffer.move_cursor(0, -1);
                        }
                    }
                    Event::Key(Down) => {
                        if let Some(buffer) = &mut self.buffer {
                            buffer.move_cursor(0, 1);
                        }
                    }
                    Event::Key(Left) => {
                        if let Some(buffer) = &mut self.buffer {
                            buffer.move_cursor(-1, 0);
                        }
                    }
                    Event::Key(Right) => {
                        if let Some(buffer) = &mut self.buffer {
                            buffer.move_cursor(1, 0);
                        }
                    }
                    evt => match &self.mode {
                        Command | View => match evt {
                            Event::Key(Char(c)) => self.command_push_char(c),
                            Event::Key(Enter) => self.command_commit(),
                            Event::Key(Esc) => self.set_mode(Mode::View),
                            Event::Key(Delete) | Event::Key(Backspace) => {
                                self.command_buffer.pop();
                            }
                            x => log!(format!("{:?}", x)),
                        },
                        Insert => match evt {
                            Event::Key(Char(c)) => {
                                if let Some(buffer) = &mut self.buffer {
                                    buffer.insert(c);
                                }
                            }
                            Event::Key(Enter) => {
                                if let Some(buffer) = &mut self.buffer {
                                    buffer.insert_newline();
                                }
                            }
                            Event::Key(Delete) | Event::Key(Backspace) => {
                                if let Some(buffer) = &mut self.buffer {
                                    buffer.remove();
                                }
                            }
                            Event::Key(Esc) => self.set_mode(Mode::View),
                            x => log!(format!("{:?}", x)),
                        },
                        _ => unimplemented!(),
                    },
                }
                log!(self.command_buffer);
                Ok(())
            }
            _ => Err("error on recv"),
        }
    }

    pub fn command_commit(&mut self)
    {
        match self.command_buffer.as_ref() {
            "q" => self.set_mode(Mode::Exit),
            "w" => {
                if let Some(buffer) = &self.buffer {
                    // TODO: take alternative path from w arguments here
                    let path = buffer.source_path().clone().unwrap();
                    log!(format!("{:?}", buffer.write(&path)));
                }
            }
            cmd => {
                log!(format!("no action for command `{}`", cmd));
                self.set_mode(Mode::View);
            }
        }
    }

    pub fn command_push_char(&mut self, c: char)
    {
        log!(format!("got {}", c));
        match c {
            ':' => self.set_mode(Mode::Command),
            'i' => self.set_mode(Mode::Insert),
            _ => self.command_buffer.push(c),
        }
    }

    fn set_mode(&mut self, mode: Mode)
    {
        log!(format!("new mode {}", mode));
        self.mode = mode;
        self.command_buffer.clear();
    }
}
