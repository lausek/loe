pub use libloe::CursorMove;

#[derive(Debug, PartialEq)]
pub enum Event
{
    Key(Key),
    Resize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Key
{
    Backspace,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Delete,
    Insert,
    Esc,
    Tab,
    Enter,
    AnyChar,
    Char(char),
    Ctrl(char),
}
