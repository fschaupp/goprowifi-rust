use crate::settingparser::*;

const BASE_PATH: &str = "http://10.5.5.9";
const CONTROL: &str = "/gp/gpControl";

fn request(path: &str) -> Result<String, Box<dyn std::error::Error>> {
  let resp = reqwest::blocking::get(path)?.text()?;
  Ok(String::from(resp))
}

fn call(path: &str) {
  let full_path: String = [BASE_PATH, path].concat();

  let result = request(&full_path.to_owned());

  println!("{:?}", result);
}

pub fn start_stream(settings: Settings) {
  call(&settings.services["live_stream_start"].url);
}

pub fn stop_stream(settings: Settings) {
  call(&settings.services["live_stream_stop"].url);
}

pub fn locate(settings: Settings, enabled: bool) {
  let command = settings.get_command("GPCAMERA_LOCATE_ID").unwrap();

  let mut full_path = [CONTROL, command.url.as_str(), "?p="].concat();
  full_path.push_str(match enabled {
    true => "1",
    false => "0",
  });

  call(full_path.as_str());
}

pub fn power_off(settings: Settings) {
  let command: Command = settings.get_command("GPCAMERA_POWER_ID").unwrap();
  let full_path = [CONTROL, command.url.as_str()].concat();
  call(full_path.as_str());
}
