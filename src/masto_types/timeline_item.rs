use super::serde_fns::*;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    #[serde(deserialize_with = "deserialize_time")]
    #[serde(serialize_with = "serialize_time")]
    pub created_at: i64,
    pub in_reply_to_id: Option<String>,
    pub in_reply_to_account_id: Option<String>,
    pub sensitive: bool,
    pub spoiler_text: String,
    pub visibility: String,
    // pub language: Value,
    /// the actual activitypub representation
    pub uri: Url,
    /// the link for the frontend for users to use
    pub url: Url,
    pub replies_count: i64,
    pub reblogs_count: i64,
    pub favourites_count: i64,
    #[serde(deserialize_with = "deserialize_time_optional")]
    #[serde(serialize_with = "serialize_time_optional")]
    pub edited_at: Option<i64>,
    pub content: String,
    // pub reblog: Value,
    pub account: Account,
    // pub media_attachments: Vec<Value>,
    // pub mentions: Vec<Value>,
    // pub tags: Vec<Value>,
    // pub emojis: Vec<Value>,
    // pub card: Value,
    // pub poll: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub username: String,
    /// in the format username@domain.com
    pub acct: String,
    pub display_name: String,
    // pub locked: bool,
    // pub bot: bool,
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
    // pub emojis: Vec<Value>,
    pub fields: Vec<Field>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub name: String,
    pub value: String,
    // pub verified_at: Value,
}
