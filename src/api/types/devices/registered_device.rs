use serde::{Deserialize, Serialize};
use super::device_info::DeviceInfo;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RegisteredDevice {
    pub device_id: String,
    pub info: DeviceInfo,
    pub registered_at: i64,
}