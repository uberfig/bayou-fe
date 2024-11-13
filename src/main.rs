use bayou_fe::masto_types::timeline_item::Post;
use gloo_net::http::Request;
use leptos::{
    component, create_resource, create_signal, html::div, view, For, HtmlElement, IntoView, ReadSignal, SignalGet, SignalSet, SignalUpdate
};
use serde::{Deserialize, Serialize};

#[component]
fn TimelinePost(post: Post) -> impl IntoView {
    // todo!()
    // let content = HtmlElement::inner_html(self, post.content);
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
            <div class="user-link inline">
                <a href={ format!("/@{}", post.account.acct) }>
                    <img src={ post.account.avatar.clone() } class="timeline-pfp" />
                </a>
                <a href={ format!("/@{}", post.account.acct)} class="no-decoration">
                    <div class="inline">
                        <h3 class="no-margin">{ display_name }</h3>
                        {pronouns}
                    </div>
                    <p class="no-margin">{ format!("@{}", post.account.acct) }</p>
                </a>
            </div>
            {content}

            // <p key={post.id.clone()}>
            // { vnode }
            // </p>
            // <div class={classes!("status-actions")}>
            //     <button><MessageSquare /></button>
            //     <button><Repeat /></button>
            //     <button><Star /></button>
            //     <button><Bookmark /></button>
            //     <button><Share2 /></button>
            // </div>
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

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    // see https://book.leptos.dev/async/10_resources.html
    // let (posts, set_posts) = create_signal(0);

    // let async_data = create_resource(
    //     count,
    //     // every time `count` changes, this will run
    //     |value| async move {
    //         logging::log!("loading data from API");
    //         load_data(value).await
    //     },
    // );

    let timeline = create_resource(
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

    // let fetched_posts: Vec<Post> =
    //                 Request::get("https://mastodon.social/api/v1/timelines/public")
    //                     .send()
    //                     .await
    //                     .unwrap()
    //                     .json()
    //                     .await
    //                     .unwrap();

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
