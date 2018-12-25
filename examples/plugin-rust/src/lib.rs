extern crate libloe;

use libloe::{buffer::Buffer, plugin::*};

// returns the plugins name
#[no_mangle]
extern fn name() -> &'static str {
    "plugin-rust"
}

// returns the commands we want to subscribe to
#[no_mangle]
extern fn commands() -> Vec<String> {
    vec!["greet".to_string()]
}

// called when a command is about to be executed
#[no_mangle]
extern fn dispatch(buffer: &mut Buffer, _cmd: &str) -> DispatchResult {
    let line = buffer.content.get_mut(0).unwrap();
    *line = "hello from so!".to_string();
    Ok(())
}

// called when the plugin gets destroyed
#[no_mangle]
extern fn unload() {}
