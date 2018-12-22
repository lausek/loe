#[macro_use]
extern crate lazy_static;
extern crate libloading;
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
    let mut config = Config::from_path(CONFIG_PATH)?;

    // TODO: remove
    config.plugin_path = Some("/home/lausek/Projects/loe-plugin/target/release/".to_string());

    App::new(config).with_args(std::env::args()).run().unwrap();

    Ok(())
}
