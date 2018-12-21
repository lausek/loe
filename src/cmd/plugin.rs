use std::path::Path;

use crate::buffer::Buffer;
use crate::cmd::CommandManager;

pub type PluginResult<T> = Result<T, &'static str>;

pub trait Plugin
{
    fn commands(&self) -> Vec<String>;
    fn dispatch(&mut self, buffer: &mut Buffer, cmd: &str) -> PluginResult<()>;
    fn unload(mut self);
}
