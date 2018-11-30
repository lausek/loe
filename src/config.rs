pub struct Config {}

impl Config
{
    pub fn from_path(path: &str) -> Result<Self, std::io::Error>
    {
        Ok(Config::default())
    }
}

impl Default for Config
{
    fn default() -> Self
    {
        Self {}
    }
}
