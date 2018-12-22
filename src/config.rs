#[derive(Debug, Default)]
pub struct Config
{
    pub plugin_path: Option<String>,
}

impl Config
{
    pub fn from_path(_path: &str) -> Result<Self, std::io::Error>
    {
        let config = Self::default();
        // TODO: read config from file here
        Ok(config)
    }
}
