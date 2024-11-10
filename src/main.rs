use bayou_fe::masto_types::timeline_item::Post;
use gloo_net::http::Request;
use web_sys::Node;
use yew::{prelude::*, virtual_dom::VNode};

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Post>,
    on_click: Callback<Post>,
}

#[function_component(PostsList)]
fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_click = on_click.clone();
    videos
        .iter()
        .map(|video| {
            let on_video_select = {
                let on_click = on_click.clone();
                let video = video.clone();
                Callback::from(move |_| on_click.emit(video.clone()))
            };

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
                <a href={ "" } class={classes!("post")}>
                    <hr />
                    <h3>{ video.account.display_name.clone() }</h3>
                    <p key={video.id.clone()} onclick={on_video_select}>
                    { vnode }
                    </p>
                    <hr />
                </a>
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
                let fetched_posts: Vec<Post> =
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
            <div>
                <PostsList videos={(*posts).clone()} on_click={on_video_select.clone()} />
            </div>
            { for details }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
