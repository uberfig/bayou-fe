use gloo_net::http::Request;
use leptos::prelude::{Update, WriteSignal};

use crate::{
    masto_types::status::Status,
    state::{Feed, State},
    timeline::loader::FeedPos,
};

/// OAuth: Public. Requires app token + read:statuses if the instance has disabled public preview.
///
/// use max_id to get posts older than a post, use min_id to get newer.
/// returns an array of [`crate::masto_types::status::Status`]
///
/// https://docs.joinmastodon.org/methods/timelines/#public
pub const PUBLIC_TIMELINE: &str = "/api/v1/timelines/public";

pub fn account_timeline(id: &str) -> String {
    format!("/api/v1/accounts/{}/statuses", id)
}

pub struct TimelineParams<'a> {
    /// Boolean. Show only local statuses? Defaults to false.
    pub local: Option<bool>,
    /// Boolean. Show only remote statuses? Defaults to false.
    pub remote: Option<bool>,
    /// Boolean. Show only statuses with media attached? Defaults to false.
    pub only_media: Option<bool>,
    /// String. All results returned will be lesser than this ID. In effect, sets an upper bound on results.
    pub max_id: Option<&'a str>,
    /// String. Returns results immediately newer than this ID. In effect, sets a cursor at this ID and paginates forward.
    pub min_id: Option<&'a str>,
    /// String. All results returned will be greater than this ID. In effect, sets a lower bound on results.
    pub since_id: Option<&'a str>,
    /// Integer. Maximum number of results to return. Defaults to 20 statuses. Max 40 statuses.
    pub limit: usize,
}
impl Default for TimelineParams<'_> {
    fn default() -> Self {
        Self {
            local: None,
            remote: None,
            only_media: None,
            max_id: None,
            min_id: None,
            since_id: None,
            limit: 20,
        }
    }
}

impl TimelineParams<'_> {
    pub fn new(state: &State) -> Self {
        Self::default().limit(state.limit)
    }
    pub fn local_only(mut self) -> Self {
        self.local = Some(true);
        self
    }
    pub fn remote_only(mut self) -> Self {
        self.remote = Some(true);
        self
    }
    pub fn media_only(mut self) -> Self {
        self.only_media = Some(true);
        self
    }
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
}

impl<'a> TimelineParams<'a> {
    /// gets posts older than the given id
    pub fn max_id(mut self, id: &'a str) -> Self {
        self.max_id = Some(id);
        self
    }
    pub fn min_id(mut self, id: &'a str) -> Self {
        self.min_id = Some(id);
        self
    }
    pub fn since_id(mut self, id: &'a str) -> Self {
        self.since_id = Some(id);
        self
    }
}

pub fn get_timeline_link(state: &State, params: &TimelineParams, feed: &str) -> String {
    let mut link = format!("https://{}{}", &state.domain, feed);
    link = apply_params(link, params);
    link
}

fn apply_params(link: String, params: &TimelineParams) -> String {
    let mut link = format!("{}?limit={}", link, params.limit);
    if let Some(max_id) = params.max_id {
        link = format!("{}&max_id={}", link, max_id);
    }
    if let Some(min_id) = params.min_id {
        link = format!("{}&min_id={}", link, min_id);
    }
    if let Some(local) = params.local {
        link = format!("{}&local={}", link, local);
    }
    if let Some(remote) = params.remote {
        link = format!("{}&remote={}", link, remote);
    }
    if let Some(only_media) = params.only_media {
        link = format!("{}&only_media={}", link, only_media);
    }
    if let Some(since_id) = params.since_id {
        link = format!("{}&since_id={}", link, since_id);
    }
    link
}

pub async fn fetch_posts(segment_link: String, set_oldest: WriteSignal<FeedPos>) -> Vec<Status> {
    let fetched_posts: Vec<Status> = Request::get(&segment_link)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    set_oldest.update(|x| {
        match fetched_posts.last() {
            Some(post) => x.oldest_id = Some(post.id.clone()),
            None => x.end_of_feed = true,
        };
    });
    fetched_posts
}
