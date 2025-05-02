use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceInfo {
    pub device_name: Option<String>,
    pub software: Option<String>,
    pub webpage: Option<String>,
    pub redirect_url: Option<String>,
}