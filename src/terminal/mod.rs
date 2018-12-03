pub mod rustbox;

use self::rustbox::RustBoxColor;
use self::rustbox::RustBoxStyle;
use crate::input::Event;

pub type Position = (isize, isize);
pub type Color = (RustBoxColor, RustBoxColor);
pub type Style = RustBoxStyle;

pub trait Terminal
{
    fn set_cursor(&self, x: isize, y: isize);
    fn size(&self) -> (usize, usize);
    fn listen(&self) -> Option<Event>;
    fn print(&self, position: Position, style: Style, color: Color, content: &str);
    fn present(&self);
}
