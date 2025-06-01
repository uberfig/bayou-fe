use codes_iso_639::part_1::LanguageCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::text_format::TextFormat;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Messageinfo {
    /// we seperate is reply and in reply to
    /// so that if a message is in reply to something
    /// but the origional is deleted or not federated
    /// clients can just say in reply to "removed or
    /// not federated"
    pub is_reply: bool,
    pub in_reply_to: Option<Uuid>,
    /// users can optionally have proxies that behave
    /// like pluralkit. Users may only use proxies that
    /// they created and clients can decide how to display
    /// proxy messages
    pub proxy_id: Option<Uuid>,
    pub content: String,
    pub format: TextFormat,
    pub language: Option<LanguageCode>,
    pub room: Uuid,
}