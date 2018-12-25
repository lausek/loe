use crate::buffer::Buffer;
use crate::plugin::{Plugin, PluginResult};

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
    fn name(&self) -> &'static str
    {
        // TODO: lookup global variable in plugin?
        "StandardPlugin"
    }

    fn commands(&self) -> Vec<String>
    {
        vec!["h", "l", "j", "k", "0", "$", "gg", "G", "H", "M", "L"]
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
            "gg" => buffer.move_cursor(Absolute(0, 0)),
            "G" => {
                let last = (buffer.content_len() - 1) as i64;
                buffer.move_cursor(Absolute(0, last));
            }
            other => log!("cmd undefined: {}", other),
        }
        Ok(())
    }

    fn unload(mut self)
    {
    }
}
