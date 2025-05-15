use super::device_info::DeviceInfo;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RegisteredDevice {
    pub device_id: Uuid,
    pub info: DeviceInfo,
    pub registered_at: i64,
}
