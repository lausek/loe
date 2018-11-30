pub mod rustbox;

use crate::input::Event;
use self::rustbox::RustBoxStyle;

pub type Position = (isize, isize);
pub type Style = RustBoxStyle;

pub trait Terminal
{
    fn set_cursor(&self, x: isize, y: isize);
    fn size(&self) -> (usize, usize); 
    fn listen(&self) -> Option<Event>;
    fn print(&self, position: Position, style: Style, content: &str);
    fn present(&self);
}
