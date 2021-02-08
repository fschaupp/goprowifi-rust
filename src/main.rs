use std::fs::File;
use std::io::Read;
use std::io::Result;

extern crate serde_json as json;
use json::to_string;

mod settingparser;
use settingparser::Settings;

// dependencies/goprowifihack/HERO5/gpControl-HERO5Session.json

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file: Result<File> = File::open(&mut args[1].to_string());
    let string: String = read_file_to_string(&mut file.unwrap());

    let settings: Settings = json::from_str(&string).unwrap();

    println!("{}", to_string(&settings).unwrap());
    println!(r#""#);
}

fn read_file_to_string(file: &mut File) -> String {
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    return content;
}
