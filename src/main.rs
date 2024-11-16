use bayou_fe::masto_types::timeline_item::Post;
use gloo_net::http::Request;
use leptos::{
    component, create_resource, create_signal, html::div, view, For, IntoView, ReadSignal,
    Resource, SignalGet, SignalSet, SignalUpdate, WriteSignal,
};
use leptos_lucide_icons::{Bookmark, MessageSquare, Repeat, Share2, Star};

#[component]
fn TimelinePost(post: Post) -> impl IntoView {
    let content = div().inner_html(post.content);

    let display_name = match &post.account.display_name.is_empty() {
        true => post.account.username,
        false => post.account.display_name,
    };

    let mut pronouns = None;
    for prop in &post.account.fields {
        if prop.name.eq_ignore_ascii_case("Pronouns") {
            pronouns = Some(view! {
                <p class=("pronouns", true) class=("no-margin", true)>{ prop.value.clone() }</p>
            })
        }
    }

    view! {
        <div class="post">
        <hr />
            <a href={ format!("/@{}", post.account.acct) } class="user-link inline no-decoration">
                    <img src={ post.account.avatar.clone() } class="timeline-pfp" />
                <div href={ format!("/@{}", post.account.acct)} class="no-decoration">
                    <div class="inline">
                        <h3 class="no-margin">{ display_name }</h3>
                        {pronouns}
                    </div>
                    <p class="no-margin">{ format!("@{}", post.account.acct) }</p>
                </div>
            </a>
            {content}

            <div class="status-actions">
                <button><MessageSquare/></button>
                <button><Repeat /></button>
                <button><Star /></button>
                <button><Bookmark /></button>
                <button><Share2 /></button>
            </div>
        <hr />
        </div>
    }
}

#[component]
fn TimelineSegment(posts: Vec<Post>) -> impl IntoView {
    view! {
        <For
            each=move || posts.clone()
            key=|post| post.id.clone()
            children=move |post: Post| {
                view! {
                  <TimelinePost post=post/>
                }
              }
        />
    }
}

pub struct Segment {
    pub contents: Resource<(), Vec<Post>>,
    pub id: String,
}

#[component]
fn LoadOlder(
    oldest: ReadSignal<Option<String>>,
    set_oldest: WriteSignal<Option<String>>,
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
                            let curr_oldest: Option<String> = oldest.get();
                            let curr_oldest = curr_oldest.unwrap_or(state.get().get_timeline());
                            let tmp_oldest = curr_oldest.clone();

                            let posts = create_resource(|| (), move |_| {
                            let value = curr_oldest.clone();
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
                                        *x = match fetched_posts.last() {
                                            Some(post) => Some(post.id.clone()),
                                            None => None,
                                        };
                                    });
                                    fetched_posts
                            }
                            });

                            segments.update(|x| {
                                x.push(Segment { contents: posts, id: tmp_oldest });
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
pub struct State {
    domain: String,
    limit: usize,
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
    let (segments, set_segments) = create_signal(Vec::<leptos::Resource<(), Vec<Post>>>::new());
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
