pub mod rustbox;

use self::rustbox::RustBoxColor;
use self::rustbox::RustBoxStyle;
use crate::input::Event;

pub type Position = (i64, i64);
pub type Color = (RustBoxColor, RustBoxColor);
pub type Style = RustBoxStyle;

pub trait Terminal
{
    fn set_cursor(&self, x: i64, y: i64);
    fn size(&self) -> (usize, usize);
    fn listen(&self) -> Option<Event>;
    fn print(&self, position: Position, style: Style, color: Color, content: &str);
    fn present(&self);
    fn clear(&self);
}
