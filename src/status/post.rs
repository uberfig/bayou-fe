use leptos::{
    component,
    html::{div, h3, p, InnerHtmlAttribute},
    prelude::*,
    view, IntoView,
};
use leptos_router::components::A;
// use leptos_lucide_icons::{Bookmark, MessageSquare, Repeat, Share2, Star};

use crate::{
    masto_types::status::Status, state::State, status::attachment_gallery::Attachments, timeline::source::RenderSrc
};

#[component]
pub fn TimelinePost(post: Status, with_link: bool, reply_chain: Option<Vec<Status>>) -> impl IntoView {
    let source = post.clone();
    let post = post.enrich_content();
    let state: ReadSignal<State> = use_context().expect("missing state");

    let (post, reblogged_by) = match post.reblog {
        Some(inner_post) => (*inner_post, Some(post.account)),
        None => (post, None),
    };

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

    view! {
        <article class="post">
            {reblogged}
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
            
            
        </article>
    }
}
