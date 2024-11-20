use super::serde_fns::*;
use serde::{Deserialize, Serialize};
use url::Url;

/// represents a post from an account
/// 
/// https://docs-p.joinmastodon.org/entities/Status/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    /// ID of the status in the database.
    pub id: String,
    /// the actual activitypub representation
    pub uri: Url,
    /// the link for the frontend for users to use
    pub url: Url,
    /// The date when this status was created.
    /// 
    /// parsed from String (ISO 8601 Datetime) however 
    /// we parse it as rfc3339 since thats what chrono has
    #[serde(deserialize_with = "deserialize_time")]
    #[serde(serialize_with = "serialize_time")]
    pub created_at: i64,

    pub account: Account,
    /// HTML-encoded status content.
    /// should be sanatized by the server
    pub content: String,

    /// Visibility of this status.
    /// See [`Visibility`]
    pub visibility: Visibility,

    pub sensitive: bool,
    /// Subject or summary line, below which status content is collapsed until expanded.
    pub spoiler_text: String,

    pub in_reply_to_id: Option<String>,
    pub in_reply_to_account_id: Option<String>,
    
    
    
    pub replies_count: i64,
    pub reblogs_count: i64,
    pub favourites_count: i64,
    #[serde(deserialize_with = "deserialize_time_optional")]
    #[serde(serialize_with = "serialize_time_optional")]
    pub edited_at: Option<i64>,
    
    // pub reblog: Value,
    
    // pub media_attachments: Vec<Value>,
    // pub mentions: Vec<Value>,
    // pub tags: Vec<Value>,
    // pub emojis: Vec<Value>,
    // pub card: Value,
    // pub poll: Value,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    /// Visible to everyone, shown in public timelines.
    Public,
    /// Visible to public, but not included in public timelines.
    Unlisted,
    /// Visible to followers only, and to any mentioned users.
    Private,
    /// Visible only to mentioned users.
    Direct,
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
pub struct Field {
    pub name: String,
    pub value: String,
    // pub verified_at: Value,
}

/// https://docs-p.joinmastodon.org/entities/Status/#Mention
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mention {
    /// The account ID of the mentioned user.
    pub id: String,
    /// The username of the mentioned user.
    pub username: String,
    /// The location of the mentioned user’s profile.
    pub url: Url,
    /// The webfinger acct: URI of the mentioned user. Equivalent to `username` for local users, or `username@domain` for remote users.
    pub acct: String,
}