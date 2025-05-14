
use reactive_stores::Store;
use serde::{Deserialize, Serialize};
use crate::api::types::{auth_token::AuthToken, devices::registered_device::RegisteredDevice};

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