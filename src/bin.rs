#![feature(result_map_or_else)]
#![allow(clippy::string_lit_as_bytes)]

extern crate lazy_static;
extern crate libloading;
extern crate rustbox;
extern crate serde_derive;
extern crate toml;

#[macro_use]
mod macros;
mod app;
mod buffer;
mod config;
mod input;
mod mode;
mod plugin;
mod terminal;
mod view;

use self::app::App;
use self::config::Config;

const CONFIG_PATH: &str = "~/.config/loe/config";

fn main()
{
    // TODO: try reading this from program arguments
    let config_path = CONFIG_PATH;
    let config = Config::from_path(config_path).unwrap_or_else(|err| {
        log!("could not load configuration from `{}`", config_path);
        log!("got error: {:?}", err);
        Config::default()
    });

    log!("{:?}", config);

    App::new(config).with_args(std::env::args()).run().unwrap();
}
