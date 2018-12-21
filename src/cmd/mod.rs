mod manager;
mod plugin;

use std::path::Path;
use std::rc::Rc;

use crate::buffer::Buffer;
use crate::cmd::plugin::PluginResult;

pub use self::manager::CommandManager;
pub use self::plugin::Plugin;

pub struct StandardPlugin {}

impl StandardPlugin
{
    pub fn load() -> Box<Self>
    {
        Box::new(Self {})
    }
}

impl Plugin for StandardPlugin
{
    fn commands(&self) -> Vec<String>
    {
        vec!["h", "l", "j", "k", "0", "$"]
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
    }

    fn dispatch(&mut self, buffer: &mut Buffer, cmd: &str) -> PluginResult<()>
    {
        use crate::input::CursorMove::*;
        match cmd {
            "h" => buffer.move_cursor(Relative(-1, 0)),
            "l" => buffer.move_cursor(Relative(1, 0)),
            "j" => buffer.move_cursor(Relative(0, 1)),
            "k" => buffer.move_cursor(Relative(0, -1)),
            "0" => buffer.move_cursor(CurrentRow(0)),
            "$" => buffer.move_cursor(CurrentRow(i64::max_value())),
            _ => log!("hello from standard plugin"),
        }
        Ok(())
    }

    fn unload(mut self)
    {
    }
}

pub struct ExternPlugin {}

impl ExternPlugin
{
    pub fn load<T>(p: &Path) -> PluginResult<Box<Self>>
    {
        unimplemented!();
    }
}

impl Plugin for ExternPlugin
{
    fn commands(&self) -> Vec<String>
    {
        vec![]
    }

    fn dispatch(&mut self, buffer: &mut Buffer, cmd: &str) -> PluginResult<()>
    {
        Ok(())
    }

    fn unload(mut self)
    {
    }
}
