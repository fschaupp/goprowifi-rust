use goprowifihack::api::*;
use goprowifihack::settingparser::*;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static MOCK_RESPONSE: RefCell<Option<Result<String, isahc::Error>>> = RefCell::new(None);
}

// Mock the HTTP request for testing
pub fn mock_request(_path: &str) -> Result<String, isahc::Error> {
    MOCK_RESPONSE.with(|response| {
        response
            .borrow()
            .clone()
            .unwrap_or(Ok("mock_response".to_string()))
    })
}

fn set_mock_response(response: Option<Result<String, isahc::Error>>) {
    MOCK_RESPONSE.with(|mock| {
        *mock.borrow_mut() = response;
    });
}

fn create_test_settings() -> Settings {
    let mut services = HashMap::new();
    services.insert(
        "live_stream_start".to_string(),
        Service {
            version: 1,
            description: "Start live stream".to_string(),
            url: "/live/start".to_string(),
        },
    );
    services.insert(
        "live_stream_stop".to_string(),
        Service {
            version: 1,
            description: "Stop live stream".to_string(),
            url: "/live/stop".to_string(),
        },
    );

    let commands = vec![
        Command {
            key: "GPCAMERA_LOCATE_ID".to_string(),
            display_name: "Locate".to_string(),
            url: "/camera/locate".to_string(),
            widget_type: "button".to_string(),
            deprecated: false,
            network_types: vec!["wifi".to_string()],
        },
        Command {
            key: "GPCAMERA_POWER_ID".to_string(),
            display_name: "Power".to_string(),
            url: "/camera/power/off".to_string(),
            widget_type: "button".to_string(),
            deprecated: false,
            network_types: vec!["wifi".to_string()],
        },
        Command {
            key: "GPCAMERA_MODE".to_string(),
            display_name: "Mode".to_string(),
            url: "/camera/mode".to_string(),
            widget_type: "button".to_string(),
            deprecated: false,
            network_types: vec!["wifi".to_string()],
        },
        Command {
            key: "GPCAMERA_SUBMODE".to_string(),
            display_name: "Submode".to_string(),
            url: "/camera/submode".to_string(),
            widget_type: "button".to_string(),
            deprecated: false,
            network_types: vec!["wifi".to_string()],
        },
    ];

    Settings {
        version: 1.0,
        schema_version: 1,
        display_hints: vec![],
        modes: vec![],
        filters: vec![],
        commands,
        status: Status::default(),
        services,
        camera_mode_map: vec![],
        info: Info::default(),
    }
}

#[test]
fn test_start_stream() {
    let settings = create_test_settings();
    start_stream(&settings);
    // Since we can't easily verify the actual HTTP request in a unit test,
    // we're mainly testing that the function doesn't panic and constructs
    // the correct URL internally
}

#[test]
fn test_stop_stream() {
    let settings = create_test_settings();
    stop_stream(&settings);
    // Similar to start_stream test
}

#[test]
fn test_locate() {
    let settings = create_test_settings();

    // Test locate enabled
    locate(&settings, true);

    // Test locate disabled
    locate(&settings, false);
}

#[test]
fn test_power_off() {
    let settings = create_test_settings();
    power_off(&settings);
}

#[test]
fn test_video_mode_settings() {
    let settings = create_test_settings();

    set_video_mode(&settings, VideoMode::Video);
    set_video_mode(&settings, VideoMode::TimeLapseVideo);
    set_video_mode(&settings, VideoMode::VideoPlusPhoto);
    set_video_mode(&settings, VideoMode::Looping);
}

#[test]
fn test_photo_mode_settings() {
    let settings = create_test_settings();

    set_photo_mode(&settings, PhotoMode::Single);
    set_photo_mode(&settings, PhotoMode::Continuous);
    set_photo_mode(&settings, PhotoMode::Night);
}

#[test]
fn test_camera_settings() {
    let settings = create_test_settings();

    set_resolution(&settings, 1080);
    set_framerate(&settings, 60);
    set_protune(&settings, true);
    set_protune(&settings, false);
}

#[test]
/// Tests command retrieval functionality with valid and invalid commands
fn test_get_command() {
    let settings = create_test_settings();

    let locate_cmd = settings.get_command("GPCAMERA_LOCATE_ID");
    assert!(locate_cmd.is_some());
    assert_eq!(locate_cmd.unwrap().url, "/camera/locate");

    let power_cmd = settings.get_command("GPCAMERA_POWER_ID");
    assert!(power_cmd.is_some());
    assert_eq!(power_cmd.unwrap().url, "/camera/power/off");

    let invalid_cmd = settings.get_command("INVALID_COMMAND");
    assert!(invalid_cmd.is_none());
}
