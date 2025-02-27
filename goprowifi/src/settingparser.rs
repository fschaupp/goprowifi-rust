extern crate miniserde as serde;

use serde::json::Value;
use serde::{json, Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result};
use std::result::Result as BasicResult;

// Main settings structure
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Settings {
    pub version: f32,
    pub schema_version: u16,
    pub display_hints: Vec<DisplayHint>,
    pub modes: Vec<Mode>,
    pub filters: Vec<Filter>,
    pub commands: Vec<Command>,
    pub status: Status,
    pub services: HashMap<String, Service>,
    pub camera_mode_map: Vec<CameraMode>,
    pub info: Info,
}

// Mode settings enums
#[derive(Debug, Clone)]
pub enum VideoMode {
    Video,
    TimeLapseVideo,
    VideoPlusPhoto,
    Looping,
}

#[derive(Debug, Clone)]
pub enum PhotoMode {
    Single,
    Continuous,
    Night,
}

#[derive(Debug, Clone)]
pub enum MultiShotMode {
    Burst,
    TimeLapsePhoto,
    NightLapse,
}

// Setting types
#[derive(Debug, Clone)]
pub enum SettingType {
    Resolution(u32),
    FrameRate(u32),
    FOV(u32),
    ProtuneOption(bool),
    WhiteBalance(u32),
    Color(u32),
    ISO(u32),
    Sharpness(u32),
    EVComp(u32),
    SpotMeter(bool),
    VideoStabilization(bool),
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct DisplayHint {
    pub key: String,
    pub display_name: String,
    pub settings: Vec<DisplayHintSettings>,
    pub commands: Vec<DisplayHintCommand>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct DisplayHintSettings {
    pub setting_id: u16,
    pub widget_type: String,
    pub precedence: u16,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct DisplayHintCommand {
    pub command_key: String,
    pub precedence: u16,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Mode {
    pub path_segment: String,
    pub display_name: String,
    pub value: u16,
    pub settings: Vec<ModeSetting>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ModeSetting {
    pub path_segment: String,
    pub display_name: String,
    pub id: u16,
    pub options: Vec<ModeSettingOption>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ModeSettingOption {
    pub display_name: String,
    pub value: u32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Filter {
    pub activated_by: Vec<Value>,
    pub blacklist: FilterBlacklist,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct FilterBlacklist {
    pub setting_id: u16,
    pub values: Vec<u16>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Command {
    pub key: String,
    pub display_name: String,
    pub url: String,
    pub widget_type: String,
    pub deprecated: bool,
    pub network_types: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Status {
    groups: Vec<StatusGroup>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct StatusGroup {
    group: String,
    fields: Vec<StatusField>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct StatusField {
    id: u16,
    name: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Service {
    pub version: u8,
    pub description: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct CameraMode {
    description: String,
    mapping_type: String,
    mode_value: u16,
    sub_mode_value: u16,
    wsdk_mode_group_key: String,
    wsdk_mode_key: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Info {
    model_number: usize,
    model_name: String,
    firmware_version: String,
    serial_number: String,
    board_type: String,
    ap_mac: String,
    ap_ssid: String,
    ap_has_default_credentials: String,
    git_sha1: String,
    capabilities: String,
}

pub fn get_settings_from(setting_file_path: String) -> BasicResult<Settings, SettingsError> {
    let mut file = File::open(setting_file_path)?;
    let string = read_file_to_string(&mut file)?;
    json::from_str(&string).map_err(SettingsError::Parse)
}

fn read_file_to_string(file: &mut File) -> Result<String> {
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

#[derive(Debug)]
pub enum SettingsError {
    Io(std::io::Error),
    Parse(miniserde::Error),
}

impl From<std::io::Error> for SettingsError {
    fn from(err: std::io::Error) -> SettingsError {
        SettingsError::Io(err)
    }
}

impl From<miniserde::Error> for SettingsError {
    fn from(err: miniserde::Error) -> SettingsError {
        SettingsError::Parse(err)
    }
}
