use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LoginErr {
    InvalidUsernameOrPassword,
    InvalidDevice,
}