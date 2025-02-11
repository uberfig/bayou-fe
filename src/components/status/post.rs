use leptos::{
    component,
    html::{div, h3, p, InnerHtmlAttribute},
    prelude::*,
    view, IntoView,
};
use leptos_router::components::A;

use crate::{
    api::masto_types::status::Status,
    components::{status::attachment_gallery::Attachments, timeline::source::RenderSrc},
    state::State,
};
// use leptos_lucide_icons::{Bookmark, MessageSquare, Repeat, Share2, Star};

#[component]
pub fn InnerPost(post: Status, with_link: bool) -> impl IntoView {
    let source = post.clone();
    let post = post.enrich_content();
    let state: ReadSignal<State> = use_context().expect("missing state");

    let attachments = match post.media_attachments {
        Some(attachments) => view! {<Attachments attachments=attachments/>}.into_any(),
        None => view! {}.into_any(),
    };

    let content = div().inner_html(post.content).attr("class", "post-body");
    let content = view! {
        {content}
        {attachments}
    };
    let content = match post.sensitive {
        true => match post.spoiler_text.is_empty() {
            true => content.into_any(),
            false => view! {
                <details>
                    <summary>{ post.spoiler_text }</summary>
                    {content}
                </details>
            }
            .into_any(),
        },
        false => content.into_any(),
    };

    let display_name = h3()
        .class("no-margin-recursive")
        .inner_html(post.account.rendered_name());

    let mut pronouns = None;
    for prop in &post.account.fields {
        if prop.name.eq_ignore_ascii_case("Pronouns") {
            pronouns = Some(
                p().class("pronouns no-margin")
                    .inner_html(prop.value.clone()),
            )
        }
    }

    view! {
        <header class="user-link no-decoration">
            <A href={ format!("/@{}", post.account.acct) }>
                <div class="user-link flex-row gap-1em no-decoration">
                    <img src={ post.account.avatar.clone() } class="timeline-pfp pfp" />
                    <div class="no-decoration user-link-text">
                        <div class="flex-row gap-1em">
                            { display_name }
                            {pronouns}
                        </div>
                        <p class="no-margin-recursive">{ format!("@{}", post.account.acct) }</p>
                    </div>
                </div>
            </A>
        </header>
        {content}

        // <div class="status-actions">
        //     <button><MessageSquare/></button>
        //     <button><Repeat /></button>
        //     <button><Star /></button>
        //     <button><Bookmark /></button>
        //     <button><Share2 /></button>
        // </div>
        {move || {
            let render = state.get().show_src;
            view! {
                <RenderSrc
                    render=render
                    src=serde_json::to_string_pretty(&source).unwrap()
                />
            }
        }}
    }
}

#[component]
pub fn TimelinePost(
    post: Status,
    with_link: bool,
    reply_chain: Option<Vec<Status>>,
    render_chain: bool,
) -> impl IntoView {
    let (post, reblogged_by) = match post.reblog {
        Some(inner_post) => (*inner_post, Some(post.account)),
        None => (post, None),
    };

    let reblogged = match reblogged_by {
        Some(account) => Some(view! {
            <div class="no-decoration reblog">
                <A href={ format!("/@{}", account.acct) }>
                    <div class="flex-row gap-1em">
                        <img src={ account.avatar.clone() } class="pfp reblog-pfp" />
                        <div class="no-decoration flex-row gap-1em">
                            { h3().class("boost-text").inner_html(account.rendered_name()) }
                            <h3 class="boost-text">{"boosted"}</h3>
                        </div>
                    </div>
                </A>
            </div>
        }),
        None => None,
    };

    let topper = match post.in_reply_to_id.clone() {
        Some(id) => match reply_chain {
            Some(reply_chain) => {
                let more_reply = match reply_chain.last() {
                    Some(more) => match &more.in_reply_to_id {
                        Some(reply) => {
                            view! {<a href=format!("/notes/{}", reply)>{"in reply"}</a>}.into_any()
                        }
                        None => view! {}.into_any(),
                    },
                    None => view! {}.into_any(),
                };
                let amount = reply_chain.len();
                let mut replies: Vec<_> = Vec::with_capacity(amount);
                let mut reply_chain = reply_chain.into_iter().peekable();
                while let Some(reply) = reply_chain.next() {
                    let above = reply_chain.peek();
                    if let Some(above) = above {
                        if above.account.id.eq(&reply.account.id) {
                            replies.push(
                                view! {
                                    <div class="no-topper">
                                        <hr />
                                        <InnerPost post=reply with_link=true />
                                    </div>
                                }
                                .into_any(),
                            );
                            continue;
                        }
                    }
                    replies.push(view! {<InnerPost post=reply with_link=true />}.into_any());
                }
                replies.reverse();

                view! {
                    <div class="reply-chain">
                        {more_reply}
                        {replies}
                        {reblogged}
                    </div>
                }
                .into_any()
            }
            None => view! {
                <a href=format!("/notes/{}", id)>{"in reply"}</a>
                {reblogged}
            }
            .into_any(),
        },
        None => reblogged.into_any(),
    };

    view! {
        <article class="post">
            {topper}
            <InnerPost post=post with_link=with_link />
        </article>
    }
}
