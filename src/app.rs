use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;

use crate::buffer::{Buffer, Buffers};
use crate::config::Config;
use crate::input::{Event, Key, Key::*};
use crate::mode::{Mode, Mode::*};
use crate::terminal::rustbox::RustBoxTerminal;
use crate::terminal::{Style, Terminal};

pub struct App
{
    pub(crate) mode: Mode,
    pub(crate) command_buffer: String,
    config: Config,
    //buffers: Buffers,
    buffer: Option<Buffer>,
    current_buffer: Option<String>,
    margin: (isize, isize),
    view: Arc<Terminal + Sync + Send>,
    events: Receiver<Event>,
}

impl App
{
    pub fn new(config: Config) -> Self
    {
        let (sender, receiver) = channel();

        let app = Self {
            config,
            //buffers: Buffers::new(),
            buffer: None,
            current_buffer: None,
            command_buffer: String::new(),
            margin: (5, 0),
            view: Arc::new(RustBoxTerminal::new()),
            events: receiver,
            mode: Mode::View,
        };

        let mut r = Arc::clone(&app.view);
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
        //.map(|arg| {
        //    //self.buffers.open(arg.as_ref());
        //    //self.current_buffer = Some(arg);
        //});
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
        if let Some(buffer) = &self.buffer {
            let style = rustbox::RB_NORMAL;
            let color = (rustbox::Color::White, rustbox::Color::Black);
            let offset = 0;
            let (w, h) = {
                let size = self.view.size();
                (size.0, size.1 - 1)
            };

            for i in 0..h {
                if let Some(line) = buffer.at(i + offset) {
                    self.view
                        .print((self.margin.0, i as isize), style, color, line);
                    self.view
                        .print((0, i as isize), style, color, format!(" {}", i).as_ref());
                } else {
                    break;
                }
            }

            let (cx, cy) = buffer.cursor();
            self.view.set_cursor(self.margin.0 + cx, cy);

            let status_color = (rustbox::Color::Black, rustbox::Color::Green);
            let status_text = match &self.mode {
                Command => format!(":{} >> {}c {}r", self.command_buffer, cx, cy),
                _ => format!("{} >> {}c {}r", self.mode, cx, cy),
            };
            self.view
                .print((0, h as isize), style, status_color, status_text.as_ref());
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
                    evt => match self.mode {
                        Command | View => match evt {
                            Event::Key(Char(c)) => self.command_push_char(c),
                            Event::Key(Enter) => self.command_commit(),
                            Event::Key(Esc) => self.set_mode(Mode::View),
                            Event::Key(Delete) | Event::Key(Backspace) => {
                                self.command_buffer.pop();
                            }
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
