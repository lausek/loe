use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;

use crate::buffer::{Buffer, Buffers};
use crate::config::Config;
use crate::input::{Event, Key, Key::*};
use crate::mode::Mode;
use crate::terminal::rustbox::RustBoxTerminal;
use crate::terminal::{Style, Terminal};

pub struct App
{
    config: Config,
    //buffers: Buffers,
    buffer: Option<Buffer>,
    current_buffer: Option<String>,
    command_buffer: String,
    margin: (isize, isize),
    view: Arc<Terminal + Sync + Send>,
    events: Receiver<Event>,
    mode: Mode,
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
            let offset = 0;
            let (w, h) = {
                let size = self.view.size();
                (size.0, size.1-1)
            };

            for i in 0..h {
                if let Some(line) = buffer.at(i+offset) {
                    self.view.print((self.margin.0, i as isize), style, line);
                    self.view.print((0, i as isize), style, format!(" {}", i).as_ref());
                } else {
                    break;
                }
            }

            let (cx, cy) = buffer.cursor();
            self.view.set_cursor(self.margin.0+cx, cy);

            let status_style = Style::from_256color(rustbox::Color::Green);
            let status_text = format!("{} >> {}c {}r", self.mode, cx, cy);
            self.view.print((0, h as isize), status_style, status_text.as_ref());
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
                    Event::Key(Esc) => self.command_buffer.clear(), 
                    Event::Key(Enter) => {
                        match self.command_buffer.as_ref() {
                            ":q" => self.mode = Mode::Exit,
                            _ => {},
                        }
                        self.command_buffer.clear();
                    },
                    Event::Key(Char(c)) => match c {
                        _ => self.command_buffer.push(c),
                    }
                    _ => {}
                }
                log!(self.command_buffer);
                Ok(())
            }
            _ => Err("error on recv"),
        }
    }
}
