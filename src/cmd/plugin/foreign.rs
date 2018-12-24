use std::path::Path;

use libloading::{Library, Symbol};

use crate::buffer::Buffer;
use crate::cmd::plugin::{Plugin, PluginResult};

pub type CommandsCallback = unsafe extern "C" fn() -> Vec<String>;
pub type DispatchCallback = unsafe extern "C" fn(&mut Buffer, &str) -> libloe::DispatchResult;

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
            let plugin = Box::new(Self { library });

            Ok(plugin)
        } else {
            Err("plugin could not be loaded".to_string())
        }
    }
}

impl Plugin for ForeignPlugin
{
    fn name(&self) -> &'static str
    {
        // TODO: lookup global variable in plugin?
        "ForeignPlugin"
    }

    fn commands(&self) -> Vec<String>
    {
        unsafe {
            self.library
                .get::<Symbol<CommandsCallback>>(b"commands")
                .map_or_else(|_| vec![], |commands| commands())
        }
    }

    fn dispatch(&mut self, buffer: &mut Buffer, cmd: &str) -> PluginResult<()>
    {
        unsafe {
            self.library
                .get::<Symbol<DispatchCallback>>(b"dispatch")
                .map_or_else(
                    |_| Err(format!("no dispatch function in plugin `{}`", self.name())),
                    |dispatch| dispatch(buffer, cmd),
                )
        }
    }

    fn unload(mut self)
    {
    }
}
