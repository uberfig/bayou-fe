use serde::{Deserialize, Serialize};
use url::Url;

use super::custom_emoji::CustomEmoji;
use super::serde_fns::*;

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
    #[serde(deserialize_with = "deserialize_time")]
    #[serde(serialize_with = "serialize_time")]
    pub created_at: i64,
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
    pub emojis: Vec<CustomEmoji>,
    pub fields: Vec<Field>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub value: String,
    #[serde(deserialize_with = "deserialize_time_optional")]
    #[serde(serialize_with = "serialize_time_optional")]
    pub verified_at: Option<i64>,
}


impl Account {
    pub fn parse_emoji(mut self) -> Self {
        self.note = CustomEmoji::parse_emoji(&self.emojis, &self.note);
        self.display_name = CustomEmoji::parse_emoji(&self.emojis, &self.display_name);
        return self;
    }
    pub fn parse_tags(mut self) -> Self {
        self
    }
    pub fn parse_mentons(mut self) -> Self {
        self
    }
    pub fn enrich_content(mut self) -> Self {
        self = self.parse_emoji();
        self = self.parse_tags();
        self = self.parse_mentons();
        self
    }
}