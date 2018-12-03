use crate::app::App;

pub enum Mode
{
    View,
    Insert,
    Command,
    Exit,
}

impl std::fmt::Display for Mode
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
    {
        match self {
            Command => write!(f, "CMD"),
            Insert => write!(f, "INSERT"),
            View => write!(f, "VIEW"),
            _ => write!(f, ""),
        }
    }
}
