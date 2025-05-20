use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Room {
    pub id: Uuid,
    pub external_id: Uuid,
    pub domain: String,
    pub community: Option<Uuid>,
    pub system_channel: bool,
    pub created: i64,
    pub known_complete: bool,
    pub is_dm: bool,
    /// if this is a dm or group chat this is the user that started it
    /// we do this so that it can be automatically deleted if that user
    /// is deleted and we are able to query for dms
    pub user_a: Option<Uuid>,
    /// only used for direct messages, user B will not be the one to
    /// init the chat. exists so it will be auto deleted if they
    /// are deleted and so they can query for dms
    pub user_b: Option<Uuid>,
    pub info: RoomInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoomInfo {
    pub name: String,
    pub description: Option<String>,
    pub category: Option<Uuid>,
    pub display_order: i64,
}
