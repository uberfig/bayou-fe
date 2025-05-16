use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct SignupUser {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub token: Option<Uuid>,
    pub application_message: Option<String>,
}
