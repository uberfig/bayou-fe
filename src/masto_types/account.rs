use serde::{Deserialize, Serialize};
use url::Url;

use super::custom_emoji::CustomEmoji;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub username: String,
    /// in the format username@domain.com
    pub acct: String,
    pub display_name: String,
    // pub locked: bool,
    pub bot: bool,
    // pub discoverable: bool,
    // pub indexable: bool,
    // pub group: bool,
    pub created_at: String,
    pub note: String,
    /// the link for the frontend for users to use
    pub url: Url,
    /// the actual activitypub representation
    pub uri: Url,
    pub avatar: String,
    pub avatar_static: String,
    pub header: String,
    pub header_static: String,
    pub followers_count: i64,
    pub following_count: i64,
    pub statuses_count: i64,
    pub last_status_at: String,
    // pub hide_collections: bool,
    pub emojis: Option<Vec<CustomEmoji>>,
    pub fields: Vec<Field>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub value: String,
    // pub verified_at: Value,
}
