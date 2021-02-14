extern crate miniserde as json;

pub mod settingparser;
use settingparser::{get_settings_from, Settings};

mod api;
use api::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let settings: Settings = get_settings_from(args[1].to_string());

    match args[2].as_str() {
        "start" => start_stream(settings),
        "stop" => stop_stream(settings),
        "locate" => locate(settings, args[3].eq("1")),
        "off" => power_off(settings),
        _ => println!("invalid command!"),
    }
}
