#[macro_use]
extern crate lazy_static;
extern crate rustbox;

#[macro_use]
mod macros;
mod app;
mod buffer;
mod cmd;
mod config;
mod input;
mod mode;
mod terminal;
mod view;

use self::app::App;
use self::config::Config;

const CONFIG_PATH: &'static str = "~/.config/loe";

fn main() -> Result<(), std::io::Error>
{
    let config = Config::from_path(CONFIG_PATH)?;

    App::new(config).with_args(std::env::args()).run();

    Ok(())
}
