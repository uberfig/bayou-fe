use bayou_fe::masto_types::timeline_item::Post;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::Node;
use yew::{prelude::*, virtual_dom::VNode};
use yew_feather::{Bookmark, MessageSquare, Repeat, Share2, Star};



#[derive(Properties, PartialEq, Deserialize, Serialize, Clone)]
struct PostWrap {
    #[serde(flatten)]
    post: Post,
}

#[function_component]
fn TimelinePost(post: &PostWrap) -> Html {
    let post = &post.post;
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    div.set_inner_html(&post.content);

    let node = Node::from(div);
    let vnode = VNode::VRef(node);

    let mut pronouns = None;
    for prop in &post.account.fields {
        if prop.name.eq_ignore_ascii_case("Pronouns") {
            pronouns = Some(html! {
                <p class={classes!("pronouns", "no-margin")}>{ prop.value.clone() }</p>
            })
        }
    }

    let display_name = match &post.account.display_name.is_empty() {
        true => post.account.username.clone(),
        false => post.account.display_name.clone(),
    };

    html! {
        <div class={classes!("post")}>
            <hr />
            <div class={classes!("user-link", "inline")}>
                <a href={ format!("/@{}", post.account.acct) }>
                    <img src={ post.account.avatar.clone() } class={classes!("timeline-pfp")} />
                </a>
                <a href={ format!("/@{}", post.account.acct)} class={classes!("no-decoration")}>
                    <div class={classes!("inline")}>
                        <h3 class={classes!("no-margin")}>{ display_name }</h3>
                        { for pronouns }
                    </div>
                    <p class={classes!("no-margin")}>{ format!("@{}", post.account.acct) }</p>
                </a>
            </div>

            <p key={post.id.clone()}>
            { vnode }
            </p>
            <div class={classes!("status-actions")}>
                <button><MessageSquare /></button>
                <button><Repeat /></button>
                <button><Star /></button>
                <button><Bookmark /></button>
                <button><Share2 /></button>
            </div>
            <hr />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<PostWrap>,
    on_click_load_more: Callback<Post>,
}

#[function_component(PostsList)]
fn timeline(VideosListProps { videos, on_click_load_more: on_click_newer }: &VideosListProps) -> Html {
    let on_click = on_click_newer.clone();
    videos
        .iter()
        .map(|post| {
            // let post = post.clone();
            html! {
                <TimelinePost post={post.post.clone()} />
            }
        })
        .collect()
}

#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Post,
}

#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    div.set_inner_html(&video.content);

    let node = Node::from(div);
    let vnode = VNode::VRef(node);
    html! {
        <div>
            <h3>{ video.account.display_name.clone() }</h3>
            { vnode }
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let posts = use_state(|| vec![]);
    {
        let posts = posts.clone();
        use_effect_with((), move |_| {
            let videos = posts.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_posts: Vec<PostWrap> =
                    Request::get("https://mastodon.social/api/v1/timelines/public")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                videos.set(fetched_posts);
            });
            || ()
        });
    }

    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Post| selected_video.set(Some(video)))
    };
    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });

    html! {
        <>
            <h1>{ "M.social public timeline" }</h1>
            <div class={classes!("timeline")}>
                <PostsList videos={(*posts).clone()} on_click_load_more={on_video_select.clone()} />
            </div>
            { for details }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
