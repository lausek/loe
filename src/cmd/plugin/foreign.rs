use std::path::Path;

use libloading::{Library, Symbol};
use libloe::plugin::*;

use crate::buffer::Buffer;
use crate::cmd::plugin::{Plugin, PluginResult};

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
        unsafe {
            self.library
                .get::<Symbol<NameCallback>>(b"name")
                .map_or_else(|_| "ForeignPlugin <noname>", |name| name())
        }
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
                    |dispatch| dispatch(buffer.inner_mut(), cmd),
                )
        }
    }

    fn unload(mut self)
    {
        unsafe {
            if let Ok(unload) = self.library.get::<Symbol<UnloadCallback>>(b"unload") {
                unload();
            }
        }
    }
}
