extern crate miniserde as json;
use goprowifi::settingparser::{get_settings_from, Settings};
use goprowifi::api::*;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        // use default path to goprowifihack
        args.push(String::from("dependencies/goprowifihack/HERO5/gpControl-HERO5Session.json"));
    }
    let settings: Settings = get_settings_from(args[1].to_string()).unwrap();

    match args[2].as_str() {
        "start" => start_stream(&settings),
        "stop" => stop_stream(&settings),
        "locate" => locate(&settings, args[3].eq("1")),
        "off" => power_off(&settings),
        _ => println!("invalid command!"),
    }
}
