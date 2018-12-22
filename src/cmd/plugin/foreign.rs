use std::path::Path;

use libloading::{Library, Symbol};

use crate::buffer::Buffer;
use crate::cmd::plugin::{Plugin, PluginResult};

pub type CommandsCallback = unsafe extern "C" fn() -> Vec<String>;
pub type DispatchCallback = unsafe extern "C" fn(&mut Buffer, &str) -> u32;

pub struct ForeignPlugin
{
    library: Library,
}

impl ForeignPlugin
{
    pub fn load(path: &Path) -> PluginResult<Box<Self>>
    {
        if let Ok(library) = Library::new(path) {
            // TODO: cache library symbols
            let plugin = Box::new(Self {
                library,
            });

            Ok(plugin)
        } else {
            Err("plugin could not be loaded".to_string())
        }
    }
}

impl Plugin for ForeignPlugin
{
    fn commands(&self) -> Vec<String>
    {
        unsafe {
            let commands: Symbol<CommandsCallback> = self.library.get(b"commands").unwrap();
            commands()
        }
    }

    fn dispatch(&mut self, buffer: &mut Buffer, cmd: &str) -> PluginResult<()>
    {
        unsafe {
            let dispatch: Symbol<DispatchCallback> = self.library.get(b"dispatch").unwrap();
            dispatch(buffer, cmd);
        }
        Ok(())
    }

    fn unload(mut self)
    {
    }
}
