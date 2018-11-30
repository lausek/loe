pub enum Mode
{
    View,
    Insert,
    Exit,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Insert => write!(f, "INSERT"),
            _ => write!(f, ""),
        }
    }
}
