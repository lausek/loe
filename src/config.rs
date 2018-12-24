use std::convert::From;

use serde_derive::Deserialize;

#[derive(Debug)]
pub enum ConfigError
{
    File(std::io::Error),
    Parse(toml::de::Error),
}

pub type ConfigResult = Result<Config, ConfigError>;

#[derive(Debug, Default, Deserialize)]
pub struct Config
{
    #[serde(rename = "plugin-path")]
    pub plugin_path: Option<String>,
}

impl Config
{
    pub fn from_path(path: &str) -> ConfigResult
    {
        let mut path_buf = std::path::PathBuf::new();
        // TODO: search for more generic solution
        if path.starts_with("~") {
            path_buf.push(dirs::home_dir().unwrap());
            path_buf.push(&path[2..]);
        } else {
            path_buf.push(path);
        }
        let path = std::fs::canonicalize(path_buf)?;
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }
}

impl From<std::io::Error> for ConfigError
{
    fn from(err: std::io::Error) -> Self
    {
        ConfigError::File(err)
    }
}

impl From<toml::de::Error> for ConfigError
{
    fn from(err: toml::de::Error) -> Self
    {
        ConfigError::Parse(err)
    }
}
