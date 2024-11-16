use bayou_fe::{masto_types::timeline_item::Post, timeline::segments::{Segment, TimelineSegment}};
use gloo_net::http::Request;
use leptos::{
    component, create_resource, create_signal, view, IntoView, ReadSignal, SignalGet, SignalSet, SignalUpdate, WriteSignal,
};

#[component]
pub fn LoadOlder(
    oldest: ReadSignal<FeedPos>,
    set_oldest: WriteSignal<FeedPos>,
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
                            let feed_state = oldest.get();
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
                                    set_oldest.update(|x| {
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

#[derive(Clone)]
pub struct State {
    pub domain: String,
    pub limit: usize,
}

impl State {
    pub fn get_older(&self, post_id: &str) -> String {
        format!(
            "https://{}//api/v1/timelines/public?max_id={}&limit={}",
            &self.domain, post_id, self.limit
        )
    }
    pub fn get_timeline(&self) -> String {
        format!("https://{}/api/v1/timelines/public", &self.domain)
    }
}

#[component]
fn App() -> impl IntoView {
    let (segments, set_segments) = create_signal(Vec::<Segment>::new());
    let (oldest, set_oldest) = create_signal(None::<String>);

    let (state, set_state) = create_signal(State {
        domain: "mastodon.social".to_string(),
        limit: 20,
    });

    // see https://book.leptos.dev/async/10_resources.html
    let timeline: leptos::Resource<(), Vec<Post>> = create_resource(
        || (),
        |_| async move {
            let fetched_posts: Vec<Post> =
                Request::get("https://mastodon.social/api/v1/timelines/public")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
            fetched_posts
        },
    );

    view! {
        <div class="timeline">
        {move || match timeline.get() {
            None => view! { <p>"Loading..."</p> }.into_view(),
            Some(data) => view! { <TimelineSegment posts=data/> }.into_view()
        }}
        </div>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
