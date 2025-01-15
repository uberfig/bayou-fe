use gloo_net::http::Request;
use leptos::{
    logging::log,
    prelude::{GetUntracked, ReadSignal, Update, WriteSignal},
};

use crate::{
    masto_api::statuses::{request_status, status_request_link}, masto_types::status::Status, state::State,
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

#[derive(Debug, Clone, Copy)]
pub enum ProfileFeeds {
    Posts,
    PostsWReplies,
    Media,
}
impl ProfileFeeds {
    pub fn set_params(&self, mut params: TimelineParams) -> TimelineParams {
        match self {
            ProfileFeeds::Posts => params.exclude_replies(),
            ProfileFeeds::PostsWReplies => params,
            ProfileFeeds::Media => params.exclude_reblogs().media_only(),
        }
    }
}

pub struct TimelineParams {
    /// Boolean. Show only local statuses? Defaults to false.
    pub local: Option<bool>,
    /// Boolean. Show only remote statuses? Defaults to false.
    pub remote: Option<bool>,
    /// Boolean. Show only statuses with media attached? Defaults to false.
    pub only_media: Option<bool>,
    /// String. All results returned will be lesser than this ID. In effect, sets an upper bound on results.
    pub max_id: Option<String>,
    /// String. Returns results immediately newer than this ID. In effect, sets a cursor at this ID and paginates forward.
    pub min_id: Option<String>,
    /// String. All results returned will be greater than this ID. In effect, sets a lower bound on results.
    pub since_id: Option<String>,
    /// Integer. Maximum number of results to return. Defaults to 20 statuses. Max 40 statuses.
    pub limit: usize,
    /// account timeline only
    ///
    /// Filter out statuses in reply to a different account.
    pub exclude_replies: Option<bool>,
    /// account timeline only
    ///
    /// Filter for pinned statuses only. Defaults to false.
    /// Pinned statuses do not receive special priority in the order of the returned results.
    ///
    /// https://docs.joinmastodon.org/methods/accounts/#query-parameters-1
    pub pinned: Option<bool>,
    /// account timeline only
    ///
    /// Filter out boosts from the response.
    ///
    /// https://docs.joinmastodon.org/methods/accounts/#query-parameters-1
    pub exclude_reblogs: Option<bool>,
    /// Filter for statuses using a specific hashtag.
    ///
    /// https://docs.joinmastodon.org/methods/accounts/#query-parameters-1
    pub tagged: Option<String>,
}
impl Default for TimelineParams {
    fn default() -> Self {
        Self {
            local: None,
            remote: None,
            only_media: None,
            max_id: None,
            min_id: None,
            since_id: None,
            limit: 20,
            exclude_replies: None,
            pinned: None,
            exclude_reblogs: None,
            tagged: None,
        }
    }
}

impl TimelineParams {
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
    /// gets posts older than the given id
    pub fn max_id(mut self, id: String) -> Self {
        self.max_id = Some(id);
        self
    }
    pub fn min_id(mut self, id: String) -> Self {
        self.min_id = Some(id);
        self
    }
    pub fn since_id(mut self, id: String) -> Self {
        self.since_id = Some(id);
        self
    }
    pub fn exclude_replies(mut self) -> Self {
        self.exclude_replies = Some(true);
        self
    }
    pub fn pinned(mut self) -> Self {
        self.pinned = Some(true);
        self
    }
    pub fn exclude_reblogs(mut self) -> Self {
        self.exclude_reblogs = Some(true);
        self
    }
    pub fn tagged(mut self, tag: String) -> Self {
        self.tagged = Some(tag);
        self
    }
}

impl TimelineParams {
    pub fn get_timeline_link(&self, state: &State, feed: &str) -> String {
        let mut link = format!("https://{}{}", &state.domain, feed);
        link = self.apply_params(link);
        link
    }
    fn apply_params(&self, link: String) -> String {
        let mut link = format!("{}?limit={}", link, self.limit);
        if let Some(max_id) = &self.max_id {
            link = format!("{}&max_id={}", link, max_id);
        }
        if let Some(min_id) = &self.min_id {
            link = format!("{}&min_id={}", link, min_id);
        }
        if let Some(local) = self.local {
            link = format!("{}&local={}", link, local);
        }
        if let Some(remote) = self.remote {
            link = format!("{}&remote={}", link, remote);
        }
        if let Some(only_media) = self.only_media {
            link = format!("{}&only_media={}", link, only_media);
        }
        if let Some(since_id) = &self.since_id {
            link = format!("{}&since_id={}", link, since_id);
        }

        if let Some(exclude_replies) = &self.exclude_replies {
            link = format!("{}&exclude_replies={}", link, exclude_replies);
        }
        if let Some(pinned) = &self.pinned {
            link = format!("{}&pinned={}", link, pinned);
        }
        if let Some(exclude_reblogs) = &self.exclude_reblogs {
            link = format!("{}&exclude_reblogs={}", link, exclude_reblogs);
        }
        if let Some(tagged) = &self.tagged {
            link = format!("{}&tagged={}", link, tagged);
        }
        link
    }
}

async fn fetch_posts(segment_link: String, set_oldest: WriteSignal<FeedPos>) -> Vec<Status> {
    log!("segment_link: {}", &segment_link);
    let fetched_posts: Vec<Status> = Request::get(&segment_link)
        .send()
        .await
        .expect("invalid response")
        .json()
        .await
        .expect("deserialization error");
    set_oldest.update(|x| {
        match fetched_posts.last() {
            Some(post) => x.oldest_id = Some(post.id.clone()),
            None => x.end_of_feed = true,
        };
    });
    fetched_posts
}

pub async fn fetch_posts_with_chain(
    segment_link: String,
    set_oldest: WriteSignal<FeedPos>,
    max_depth: u32,
    state: ReadSignal<State>,
) -> Vec<(Status, Option<Vec<Status>>)> {
    let fetched_posts: Vec<Status> = fetch_posts(segment_link, set_oldest).await;
    let mut with_replies = Vec::with_capacity(fetched_posts.len());
    for post in fetched_posts {
        let replies = match post.in_reply_to_id.is_some() {
            true => {
                let mut reply_chain: Vec<Status> = Vec::new();
                let mut to_fetch = post.in_reply_to_id.clone();
                'inner: for _ in 0..max_depth {
                    match to_fetch {
                        Some(fetch_id) => {
                            log!("{}", &fetch_id);
                            let link = status_request_link(&state.get_untracked(), &fetch_id);
                            log!("{}", &link);
                            let status = request_status(link).await;
                            match status {
                                Some(status) => {
                                    to_fetch = status.in_reply_to_id.clone();
                                    reply_chain.push(status);
                                }
                                None => {
                                    log!("failed to fetch status");
                                    break 'inner;
                                },
                            }
                        }
                        None => break 'inner,
                    }
                }
                Some(reply_chain)
            }
            false => None,
        };
        with_replies.push((post, replies));
    }

    with_replies
}
