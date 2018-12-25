use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use crate::buffer::Buffer;
use crate::plugin::Plugin;

pub type SharedPlugin = Rc<Mutex<Box<dyn Plugin>>>;

pub struct CommandManager
{
    plugins: Vec<SharedPlugin>,
    commands: HashMap<String, SharedPlugin>,
}

impl CommandManager
{
    pub fn new() -> Self
    {
        Self {
            plugins: vec![],
            commands: HashMap::new(),
        }
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<(), String>
    {
        let rc_plugin = {
            let cmds = plugin.commands().into_iter();
            let rc = Rc::from(Mutex::new(plugin));
            // TODO: test if all cmds can be added without collision
            for cmd in cmds {
                log!("defining cmd {}", cmd);
                self.register_command(cmd, Rc::clone(&rc))?;
            }
            rc
        };
        self.plugins.push(rc_plugin);
        Ok(())
    }

    fn register_command<T>(&mut self, cmd: T, plugin: SharedPlugin) -> Result<(), String>
    where
        T: Into<String> + Eq + std::hash::Hash,
        String: std::borrow::Borrow<T>,
    {
        if self.commands.contains_key(&cmd) {
            return Err("command already exists".to_string());
        }
        self.commands.insert(cmd.into(), plugin);
        Ok(())
    }

    pub fn dispatch(&mut self, buffer: &mut Buffer, cmd: &str) -> Result<(), String>
    {
        if let Some(plugin) = self.commands.get_mut(cmd) {
            plugin.lock().unwrap().dispatch(buffer, cmd)
        } else {
            Err("command not found".to_string())
        }
    }
}
