use super::auth_token::AuthToken;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BearrerWithInfo<T> {
    pub info: T,
    pub token: AuthToken,
}
