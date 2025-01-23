use serde::{Deserialize, Serialize};

use super::{custom_emoji::CustomEmoji, serde_fns::*};

/// Represents a poll attached to a status.
///
/// https://docs-p.joinmastodon.org/entities/Poll/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Poll {
    /// The ID of the poll in the database.
    pub id: String,
    /// When the poll ends, or none if the poll does not end
    #[serde(deserialize_with = "deserialize_time_optional")]
    #[serde(serialize_with = "serialize_time_optional")]
    pub expires_at: Option<i64>,
    /// Is the poll currently expired?
    pub expired: bool,
    /// Does the poll allow multiple-choice answers?
    pub multiple: bool,
    /// How many votes have been received.
    pub votes_count: u64,
    /// multiple-choice poll only
    ///
    /// How many unique accounts have voted on a multiple-choice poll.
    /// none if if [`Poll::multiple`] is false.
    pub voters_count: Option<u64>,
    /// Possible answers for the poll.
    pub options: Vec<PollOption>,
    /// Custom emoji to be used for rendering poll options.
    pub emojis: Vec<CustomEmoji>,
    /// When called with a user token, has the authorized user voted?
    pub voted: Option<bool>,
    /// When called with a user token, which options has the authorized user chosen? Contains an array of index values for [`Poll::options`].
    pub own_votes: Option<Vec<u64>>,
}

/// https://docs-p.joinmastodon.org/entities/Poll/#Option
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PollOption {
    /// The text value of the poll option.
    pub title: String,
    /// The total number of received votes for this option.
    /// none if the results are not published yet
    pub votes_count: Option<u64>,
}
