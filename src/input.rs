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

pub enum CursorMove
{
    Absolute(i64, i64),
    Relative(i64, i64),
    EndOfRow(i64),
    CurrentRow(i64),
}
