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
            Mode::Command => write!(f, "CMD"),
            Mode::Insert => write!(f, "INSERT"),
            Mode::View => write!(f, "VIEW"),
            _ => write!(f, ""),
        }
    }
}
