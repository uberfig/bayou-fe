use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DBAuthToken {
    pub required_token: AuthToken,
    pub expiry: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct AuthToken {
    pub token: Uuid,
    /// auth tokens will only be valid for the device they were
    /// issued to
    pub device_id: Uuid,
    /// the uid this auth token is valid for
    pub uid: Uuid,
}
