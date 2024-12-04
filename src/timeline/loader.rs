use crate::{
    masto_types::timeline_item::Status,
    state::{Feed, State},
};
use gloo_net::http::Request;
use leptos::{
    component,
    prelude::{signal, Get, IntoAny, OnAttribute, ReadSignal, Set, Update, WriteSignal},
    view, IntoView,
};
use leptos::{prelude::ElementChild, server::LocalResource};

use super::segments::Segment;

#[derive(Clone)]
pub struct FeedPos {
    pub oldest_id: Option<String>,
    pub end_of_feed: bool,
}

impl FeedPos {
    pub fn older_posts_link(&self, state: &State, feed: Feed) -> String {
        match &self.oldest_id {
            Some(oldest) => state.get_older_link(&oldest, feed),
            None => state.get_timeline_link(feed),
        }
    }
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

#[component]
pub fn LoadOlder(
    feed_state: ReadSignal<FeedPos>,
    set_feed_state: WriteSignal<FeedPos>,
    state: ReadSignal<State>,
    segments: WriteSignal<Vec<Segment>>,
    feed: Feed,
) -> impl IntoView {
    let (loading, set_loading) = signal(false);

    view! {
        {move ||
            if !loading.get() {
                view! {
                    <button
                        on:click= move |_| {
                            set_loading.set(true);
                            let feed_state = feed_state.get();
                            let segment_link: String = feed_state.older_posts_link(&state.get(), feed);
                            let tmp_link = segment_link.clone();

                            let posts = LocalResource::new(move || {
                                let value = segment_link.clone();
                                async move {
                                    let curr_oldest = value.clone();
                                    let fetched_posts: Vec<Status> =
                                            Request::get(&curr_oldest)
                                                .send()
                                                .await
                                                .unwrap()
                                                .json()
                                                .await
                                                .unwrap();
                                        set_loading.set(false);
                                        set_feed_state.update(|x| {
                                            match fetched_posts.last() {
                                                Some(post) => x.oldest_id = Some(post.id.clone()),
                                                None => x.end_of_feed = true,
                                            };
                                        });
                                        fetched_posts
                                }
                                });

                            segments.update(|x| {
                                x.push(Segment { contents: posts, id: tmp_link });
                            });
                        }
                    >
                    "Load older"
                    </button>
                }.into_any()
            }
            else {
                view! {
                    <p>"loading..."</p>
                }.into_any()
            }
        }

    }
}
