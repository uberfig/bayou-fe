use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeviceInfo {
    pub device_name: Option<String>,
    pub software: Option<String>,
    pub webpage: Option<String>,
    pub redirect_url: Option<String>,
}

impl DeviceInfo {
    pub fn new() -> Self {
        DeviceInfo {
            device_name: None,
            software: Some("bayou-fe".to_string()),
            webpage: None,
            redirect_url: None,
        }
    }
}