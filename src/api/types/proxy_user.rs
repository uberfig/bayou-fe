use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiProxyUser {
    pub id: Uuid,
    pub name: String,
    pub bio: Option<String>,
    pub created: i64,
    pub parent_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewProxyUser {
    pub name: String,
    pub bio: Option<String>,
}