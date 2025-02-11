use crate::settingparser::{Command, ModeSetting, MultiShotMode, PhotoMode, Settings, VideoMode};
use isahc::prelude::*;

const BASE_PATH: &str = "http://10.5.5.9";
const CONTROL: &str = "/gp/gpControl";

#[cfg(not(test))]
fn request(path: &str) -> Result<String, isahc::Error> {
    let mut response = isahc::get(path)?;
    Ok(response.text()?)
}

#[cfg(test)]
fn request(path: &str) -> Result<String, isahc::Error> {
    Ok(String::from(path))
}

fn call(path: &String) {
  let full_path: String = [BASE_PATH, path.as_str()].concat();

  let result = request(&full_path.to_owned());

  println!("{:?}", result);
}

pub fn start_stream(settings: &Settings) {
  call(&settings.services["live_stream_start"].url);
}

pub fn stop_stream(settings: &Settings) {
  call(&settings.services["live_stream_stop"].url);
}

pub fn locate(settings: &Settings, enabled: bool) {
  let command = settings.get_command("GPCAMERA_LOCATE_ID").unwrap();

  let mut full_path = [CONTROL, command.url.as_str(), "?p="].concat();
  full_path.push_str(match enabled {
    true => "1",
    false => "0",
  });

  call(&full_path);
}

pub fn power_off(settings: &Settings) {
  let command: Command = settings.get_command("GPCAMERA_POWER_ID").unwrap();
  let full_path = [CONTROL, command.url.as_str()].concat();
  call(&full_path);
}

pub fn set_video_mode(settings: &Settings, mode: VideoMode) {
  settings.set_video_mode(mode);
}

pub fn set_photo_mode(settings: &Settings, mode: PhotoMode) {
  settings.set_photo_mode(mode);
}

pub fn set_resolution(settings: &Settings, resolution: u32) {
  settings.set_mode("resolution", resolution);
}

pub fn set_framerate(settings: &Settings, fps: u32) {
  settings.set_mode("fps", fps);
}

pub fn set_protune(settings: &Settings, enabled: bool) {
  settings.set_mode("protune", enabled as u32);
}

impl Settings {
  pub fn get_command(&self, key: &str) -> Option<Command> {
      self.commands.clone().into_iter().find(|e| e.key == key)
  }

  // Add helper method to get settings by ID
  pub fn get_setting(&self, setting_id: u16) -> Option<ModeSetting> {
      for mode in &self.modes {
          if let Some(setting) = mode.settings.iter().find(|s| s.id == setting_id) {
              return Some(setting.clone());
          }
      }
      None
  }

  // Add helper method to get setting value by name
  pub fn get_setting_value(&self, setting_name: &str) -> Option<u32> {
      for mode in &self.modes {
          for setting in &mode.settings {
              if setting.path_segment == setting_name {
                  return Some(setting.options[0].value);
              }
          }
      }
      None
  }

  pub fn set_video_mode(&self, mode: VideoMode) {
      let mode_value = match mode {
          VideoMode::Video => 0,
          VideoMode::TimeLapseVideo => 1,
          VideoMode::VideoPlusPhoto => 2,
          VideoMode::Looping => 3,
      };
      self.set_mode_submode("video", "current_sub_mode", mode_value);
  }

  pub fn set_photo_mode(&self, mode: PhotoMode) {
      let mode_value = match mode {
          PhotoMode::Single => 0,
          PhotoMode::Continuous => 1,
          PhotoMode::Night => 2,
      };
      self.set_mode_submode("photo", "current_sub_mode", mode_value);
  }

  pub fn set_multi_shot_mode(&self, mode: MultiShotMode) {
      let mode_value = match mode {
          MultiShotMode::Burst => 0,
          MultiShotMode::TimeLapsePhoto => 1,
          MultiShotMode::NightLapse => 2,
      };
      self.set_mode_submode("multi_shot", "current_sub_mode", mode_value);
  }

  pub fn set_mode(&self, _mode_name: &str, value: u32) {
    let command = self.get_command("GPCAMERA_MODE").unwrap();

    let mut full_path = [CONTROL, command.url.as_str(), "?p="].concat();
    full_path.push_str(value.to_string().as_str());

    call(&full_path);
  }

  pub fn set_mode_submode(&self, _mode_name: &str, _submode_name: &str, value: u32) {
    let command = self.get_command("GPCAMERA_SUBMODE").unwrap();

    let mut full_path = [CONTROL, command.url.as_str(), "?p="].concat();
    full_path.push_str(value.to_string().as_str());

    call(&full_path);
  }
}
