use crate::api::types::{auth_token::AuthToken, devices::registered_device::RegisteredDevice};
use reactive_stores::Store;
use serde::{Deserialize, Serialize};

pub const AUTH_TOKEN: &str = "auth-token";
pub const DEVICE_TOKEN: &str = "device-token";

#[derive(Clone, PartialEq, Debug, Store)]
pub struct State {
    pub prefix: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub struct PersistantState {
    pub device: Option<RegisteredDevice>,
    pub auth: Option<AuthToken>,
}

impl State {
    pub fn get_prefix(&self) -> &str {
        &self.prefix
    }
}
