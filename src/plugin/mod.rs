mod dynamic;
mod manager;
mod standard;

use crate::buffer::Buffer;

pub use self::dynamic::DynamicPlugin;
pub use self::manager::CommandManager;
pub use self::standard::StandardPlugin;

pub type PluginResult<T> = Result<T, String>;

pub trait Plugin
{
    fn name(&self) -> &'static str;
    fn commands(&self) -> Vec<String>;
    fn dispatch(&mut self, buffer: &mut Buffer, cmd: &str) -> PluginResult<()>;
    fn unload(mut self)
    where
        Self: Sized,
    {
    }
}
