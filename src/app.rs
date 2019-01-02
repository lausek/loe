use std::sync::mpsc::{channel, Receiver};
use std::thread;

use crate::buffer::Buffer;
use crate::config::Config;
use crate::input::{CursorMove::*, Event, Key::*};
use crate::mode::Mode;
use crate::plugin::{CommandManager, DynamicPlugin, Plugin, StandardPlugin};
use crate::view::View;

pub struct App
{
    pub(crate) mode: Mode,
    pub(crate) command_buffer: String,
    config: Config,
    buffer: Option<Buffer>,
    command_manager: CommandManager,
    margin: (i64, i64),
    events: Receiver<Event>,
    view: View,
}

impl App
{
    pub fn new(config: Config) -> Self
    {
        let (sender, receiver) = channel();

        let mut app = Self {
            config,
            buffer: None,
            command_buffer: String::new(),
            command_manager: CommandManager::new(),
            margin: (5, 0),
            events: receiver,
            mode: Mode::View,
            view: View::new(),
        };

        let r = app.view.terminal();
        thread::spawn(move || loop {
            if let Some(event) = r.listen() {
                sender.send(event).ok();
            }
        });

        app.command_manager
            .add_plugin(StandardPlugin::load())
            .unwrap();

        if let Some(plugin_dir_path) = app.config.plugin_path.as_ref() {
            log!("plugin_path: {}", plugin_dir_path);
            if let Ok(plugin_dir) = std::fs::read_dir(plugin_dir_path) {
                for plugin in plugin_dir {
                    if plugin.is_err() {
                        log!("skipping plugin");
                        continue;
                    }
                    let plugin_path = plugin.unwrap().path();
                    let (plugin_name, plugin_state) =
                        if let Ok(plugin) = DynamicPlugin::load(plugin_path.as_path()) {
                            let name = (*plugin).name();
                            let added = app.command_manager.add_plugin(plugin);
                            (name, if added.is_ok() { "okay" } else { "failed" })
                        } else {
                            ("<noname>", "failed")
                        };
                    log!("loading plugin {}: {:?}", plugin_name, plugin_state);
                }
            } else {
                log!("could not load plugin_path");
            }
        }

        app
    }

    pub fn with_args(mut self, mut args: std::env::Args) -> Self
    {
        if let Some(arg) = args.nth(1) {
            self.buffer = Buffer::load(&arg).or_else(|_| Buffer::create(&arg)).ok();
        }
        self
    }

    pub fn run(&mut self) -> Result<(), &'static str>
    {
        loop {
            self.render();
            self.wait_for_event()?;

            if let Mode::Exit = self.mode {
                break;
            }
        }
        self.view.clear();
        self.view.present();
        Ok(())
    }

    pub fn render(&mut self)
    {
        self.view.clear();
        if let Some(buffer) = &mut self.buffer {
            let _color = (rustbox::Color::White, rustbox::Color::Black);
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
                Mode::Command => format!(
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
                            buffer.move_cursor(Relative(0, -1));
                        }
                    }
                    Event::Key(Down) => {
                        if let Some(buffer) = &mut self.buffer {
                            buffer.move_cursor(Relative(0, 1));
                        }
                    }
                    Event::Key(Left) => {
                        if let Some(buffer) = &mut self.buffer {
                            buffer.move_cursor(Relative(-1, 0));
                        }
                    }
                    Event::Key(Right) => {
                        if let Some(buffer) = &mut self.buffer {
                            buffer.move_cursor(Relative(1, 0));
                        }
                    }
                    evt => match &self.mode {
                        Mode::Command | Mode::View => match evt {
                            Event::Key(Char(c)) => self.command_push_char(c),
                            Event::Key(Enter) => self.command_commit(),
                            Event::Key(Esc) => self.set_mode(Mode::View),
                            Event::Key(Delete) | Event::Key(Backspace) => {
                                self.command_buffer.pop();
                            }
                            x => log!(format!("{:?}", x)),
                        },
                        Mode::Insert => match evt {
                            Event::Key(Char(c)) => {
                                if let Some(buffer) = &mut self.buffer {
                                    buffer.insert(c).unwrap();
                                }
                            }
                            Event::Key(Enter) => {
                                if let Some(buffer) = &mut self.buffer {
                                    buffer.insert_newline().unwrap();
                                }
                            }
                            Event::Key(Delete) | Event::Key(Backspace) => {
                                if let Some(buffer) = &mut self.buffer {
                                    drop(buffer.remove());
                                }
                            }
                            Event::Key(Esc) => self.set_mode(Mode::View),
                            x => log!(format!("{:?}", x)),
                        },
                        _ => {}
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
        if self.buffer.is_none() {
            return;
        }
        if self
            .command_manager
            .dispatch(self.buffer.as_mut().unwrap(), &self.command_buffer)
            .is_ok()
        {
        } else {
            match self.command_buffer.as_ref() {
                "q" => {
                    self.set_mode(Mode::Exit);
                    return;
                }
                "w" => {
                    if let Some(buffer) = &self.buffer {
                        // TODO: take alternative path from w arguments here
                        let path = buffer.source_path().clone().unwrap();
                        log!(format!("{:?}", buffer.write(&path)));
                    }
                }
                cmd => log!(format!("no action for command `{}`", cmd)),
            }
        }
        self.set_mode(Mode::View);
    }

    pub fn command_push_char(&mut self, c: char)
    {
        log!("got {}", c);
        match c {
            ':' => self.set_mode(Mode::Command),
            'i' => self.set_mode(Mode::Insert),
            _ => {
                self.command_buffer.push(c);
                if self.buffer.is_some()
                    && self
                        .command_manager
                        .dispatch(self.buffer.as_mut().unwrap(), &self.command_buffer)
                        .is_ok()
                {
                    self.command_buffer.clear();
                }
            }
        }
    }

    fn set_mode(&mut self, mode: Mode)
    {
        log!("new mode {}", mode);
        self.mode = mode;
        self.command_buffer.clear();
    }
}
