use crate::{
    masto_api::timelines::{fetch_posts, TimelineParams},
    state::State,
};
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
    pub fn older_posts_link(&self, state: &State, feed: &str) -> String {
        match &self.oldest_id {
            Some(oldest) => TimelineParams::new(state)
                .max_id(oldest.to_string())
                .get_timeline_link(state, feed),
            None => TimelineParams::new(state).get_timeline_link(state, feed),
        }
    }
}

fn load_new(
    set_loading: WriteSignal<bool>,
    feed_state: ReadSignal<FeedPos>,
    set_feed_state: WriteSignal<FeedPos>,
    state: ReadSignal<State>,
    feed: String,
    segments: WriteSignal<Vec<Segment>>,
) {
    set_loading.set(true);
    let feed_state = feed_state.get();
    let segment_link: String = feed_state.older_posts_link(&state.get(), &feed);
    let tmp_link = segment_link.clone();

    let posts = LocalResource::new(move || {
        let value = segment_link.clone();
        async move {
            let set_loading = set_loading;
            let posts = fetch_posts(value.clone(), set_feed_state).await;
            set_loading.set(false);
            posts
        }
    });

    segments.update(|x| {
        x.push(Segment {
            contents: posts,
            id: tmp_link,
        });
    });
}

#[component]
pub fn LoadOlder(
    feed_state: ReadSignal<FeedPos>,
    set_feed_state: WriteSignal<FeedPos>,
    state: ReadSignal<State>,
    segments: WriteSignal<Vec<Segment>>,
    feed: String,
) -> impl IntoView {
    let (loading, set_loading) = signal(false);

    view! {
        {move ||
            if !loading.get() {
                let feed = feed.clone();
                view! {
                    <button
                        on:click= move |_| load_new(set_loading,feed_state,set_feed_state,state,feed.clone(),segments)
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
