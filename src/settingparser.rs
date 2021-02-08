extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct Settings {
  version: f32,
  schema_version: u16,
  display_hints: Vec<DisplayHint>,
  modes: Vec<Mode>,
  filters: Vec<Filter>,
  commands: Vec<Command>,
  status: Status,
  services: Value,
  #[serde(default)]
  camera_mode_map: Vec<CameraMode>,
  info: Info,
}

#[derive(Serialize, Deserialize)]
struct DisplayHint {
  key: String,
  display_name: String,
  settings: Vec<DisplayHintSettings>,
  commands: Vec<DisplayHintCommand>,
}

#[derive(Serialize, Deserialize)]
struct DisplayHintSettings {
  setting_id: u16,
  widget_type: String,
  precedence: u16,
}

#[derive(Serialize, Deserialize)]
struct DisplayHintCommand {
  command_key: String,
  precedence: u16,
}

#[derive(Serialize, Deserialize)]
struct Mode {
  path_segment: String,
  display_name: String,
  value: u16,
  settings: Vec<ModeSetting>,
}

#[derive(Serialize, Deserialize)]
struct ModeSetting {
  path_segment: String,
  display_name: String,
  id: u16,
  options: Vec<ModeSettingOption>,
}

#[derive(Serialize, Deserialize)]
struct ModeSettingOption {
  display_name: String,
  value: u32,
}

#[derive(Serialize, Deserialize)]
struct Filter {
  activated_by: Vec<Value>,
  blacklist: FilterBlacklist,
}

#[derive(Serialize, Deserialize)]
struct FilterBlacklist {
  setting_id: u16,
  values: Vec<u16>,
}

#[derive(Serialize, Deserialize)]
struct Command {
  key: String,
  display_name: String,
  url: String,
  widget_type: String,
  #[serde(default)]
  deprecated: bool,
  network_types: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Status {
  groups: Vec<StatusGroup>,
}

#[derive(Serialize, Deserialize)]
struct StatusGroup {
  group: String,
  fields: Vec<StatusField>,
}

#[derive(Serialize, Deserialize)]
struct StatusField {
  id: u16,
  name: String,
}

#[derive(Serialize, Deserialize)]
struct CameraMode {
  description: String,
  mapping_type: String,
  mode_value: u16,
  sub_mode_value: u16,
  wsdk_mode_group_key: String,
  wsdk_mode_key: String,
}

#[derive(Serialize, Deserialize)]
struct Info {
  model_number: u128,
  model_name: String,
  firmware_version: String,
  serial_number: String,
  board_type: String,
  ap_mac: String,
  ap_ssid: String,
  ap_has_default_credentials: String,
  git_sha1: String,
  #[serde(default)]
  capabilities: String,
}
