use gloo_net::http::Request;
use leptos::{
    component, create_resource, create_signal, view, IntoView, ReadSignal, Resource, SignalGet,
    SignalSet, SignalUpdate, WriteSignal,
};

use crate::{masto_types::timeline_item::Post, state::State};

use super::segments::Segment;

#[derive(Clone)]
pub struct FeedPos {
    pub oldest_id: Option<String>,
    pub end_of_feed: bool,
}

impl FeedPos {
    pub fn older_posts_link(&self, state: &State) -> String {
        match &self.oldest_id {
            Some(oldest) => state.get_older(&oldest),
            None => state.get_timeline(),
        }
    }
}

pub fn fetch_posts(
    segment_link: String,
    set_oldest: WriteSignal<FeedPos>,
) -> Resource<(), Vec<Post>> {
    create_resource(
        || (),
        move |_| {
            let value = segment_link.clone();
            async move {
                let fetched_posts: Vec<Post> = Request::get(&value)
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
        },
    )
}

#[component]
pub fn LoadOlder(
    feed_state: ReadSignal<FeedPos>,
    set_feed_state: WriteSignal<FeedPos>,
    state: ReadSignal<State>,
    segments: WriteSignal<Vec<Segment>>,
) -> impl IntoView {
    let (loading, set_loading) = create_signal(false);

    view! {
        {move ||
            if !loading.get() {
                view! {
                    <button
                        on:click= move |_| {
                            set_loading.set(true);
                            let feed_state = feed_state.get();
                            let segment_link: String = feed_state.older_posts_link(&state.get());
                            let tmp_link = segment_link.clone();

                            let posts = create_resource(|| (), move |_| {
                            let value = segment_link.clone();
                            async move {
                                let curr_oldest = value.clone();
                                let fetched_posts: Vec<Post> =
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
                }.into_view()
            }
            else {
                view! {
                    <p>"loading..."</p>
                }.into_view()
            }
        }

    }
}
