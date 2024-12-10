use std::collections::HashMap;

use super::{account::Account, custom_emoji::CustomEmoji, poll::Poll, serde_fns::*};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
    pub url: Option<Url>,
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

    /// Media that is attached to this status.
    /// See [`MediaAttachment`]
    pub media_attachments: Option<Vec<MediaAttachment>>,

    //https://docs-p.joinmastodon.org/entities/Status/#application
    //pub application
    /// Mentions of users within the status content.
    pub mentions: Vec<Mention>,

    /// Hashtags used within the status content.
    pub tags: Vec<Tag>,

    /// Custom emoji to be used when rendering status content.
    pub emojis: Vec<CustomEmoji>,

    pub in_reply_to_id: Option<String>,
    pub in_reply_to_account_id: Option<String>,

    pub replies_count: i64,
    pub reblogs_count: i64,
    pub favourites_count: i64,

    //https://docs-p.joinmastodon.org/entities/Status/#reblog
    pub reblog: Option<Box<Status>>,
    /// The poll attached to the status.
    pub poll: Option<Poll>,

    // pub card: Value,
    // pub language: Value,
    /// Plain-text source of a status.
    /// Returned instead of content when status is deleted, so the
    /// user may redraft from the source text without the client
    /// having to reverse-engineer the original text from the HTML content.
    pub text: Option<String>,

    /// Timestamp of when the status was last edited.
    #[serde(deserialize_with = "deserialize_time_optional")]
    #[serde(serialize_with = "serialize_time_optional")]
    pub edited_at: Option<i64>,
    // need to do the others for auth users after here
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl Status {
    pub fn parse_emoji(mut self) -> Self {
        self.content = CustomEmoji::parse_emoji(&self.emojis, &self.content);
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
        self.account = self.account.enrich_content();
        if let Some(reblog) = self.reblog {
            self.reblog = Some(Box::new(reblog.enrich_content()));
        }
        self
    }
}

/// Represents a file or media attachment that can be added to a status.
///
/// https://docs-p.joinmastodon.org/entities/MediaAttachment/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MediaAttachment {
    /// The ID of the attachment in the database.
    pub id: String,
    /// The type of the attachment.
    #[serde(rename = "type")]
    pub type_field: MediaType,
    /// The location of the original full-size attachment.
    pub url: Url,
    /// The location of a scaled-down preview of the attachment.
    pub preview_url: Url,
    /// The location of the full-size original attachment on the remote website.
    pub remote_url: Option<Url>,
    // // idk how this is formatted and will need to do some work to figure it out
    // // https://docs-p.joinmastodon.org/entities/MediaAttachment/#meta
    // // pub meta: String ?
    /// Alternate text that describes what is in the media attachment,
    /// to be used for the visually impaired or when media attachments do not load.
    pub description: Option<String>,
    /// A hash computed by the BlurHash algorithm,
    /// for generating colorful preview thumbnails when media has not been downloaded yet.
    pub blurhash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaType {
    /// unsupported or unrecognized file type
    Unknown,
    /// Static image
    Image,
    /// Looping, soundless animation
    Gifv,
    /// Video clip
    Video,
    /// Audio track
    Audio,
}

/// this can optionally be used if we want to allow for other
/// types to come from the server and handle it gracefully
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybeMediaType {
    MediaType(MediaType),
    Unknown(String),
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

/// https://docs-p.joinmastodon.org/entities/Status/#Tag
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// The value of the hashtag after the # sign.
    pub name: String,
    /// A link to the hashtag on the instance.
    pub url: Url,
}
