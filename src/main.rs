use bayou_fe::masto_types::timeline_item::Post;
use gloo_net::http::Request;
use leptos::{
    component, create_resource, html::div, view, For, IntoView, SignalGet
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

#[component]
fn App() -> impl IntoView {
    // see https://book.leptos.dev/async/10_resources.html
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
