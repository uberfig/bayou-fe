use leptos::{
    component,
    html::{div, h3, p, InnerHtmlAttribute},
    prelude::*,
    view, IntoView,
};
use leptos_router::components::A;
// use leptos_lucide_icons::{Bookmark, MessageSquare, Repeat, Share2, Star};

use crate::{
    masto_types::status::{MediaAttachment, Status},
    timeline::source::RenderSrc,
};

pub fn generate_attachments(attachments: Vec<MediaAttachment>) -> AnyView {
    let mut attachments = attachments.into_iter()
            .map(|attachment| {
                match attachment.type_field {
                    crate::masto_types::status::MediaType::Unknown => view! {<a href={attachment.url.to_string()}>{attachment.url.to_string()}</a>}.into_any(),
                    crate::masto_types::status::MediaType::Image => view! {
                        <img class="attachment attachment-img" src={attachment.url.to_string()} alt={attachment.description}/>
                    }.into_any(),
                    crate::masto_types::status::MediaType::Gifv => view! {
                        <video class="attachment attachment-gif" autoplay muted loop aria-label={attachment.description}>
                            <source src={attachment.url.to_string()} type="video/mp4" />
                        </video>
                    }.into_any(),
                    crate::masto_types::status::MediaType::Video => view! {<p class="attachment attachment-video" >{attachment.url.to_string()}</p>}.into_any(),
                    crate::masto_types::status::MediaType::Audio => view! {
                        <audio class="attachment attachment-audio" controls>
                            <source src={attachment.url.to_string()} type="audio" />
                            {"Your browser does not support the audio element."}
                        </audio>
                    }.into_any(),
                }
            })
            .collect::<Vec<_>>();

    match attachments.len() {
        0 => view! {}.into_any(),
        1 => view! {
            <div class="attachment-container">
                {attachments}
            </div>
        }
        .into_any(),
        2 => {
            let first = attachments.remove(0);
            let second = attachments.remove(0);
            view! {
                <div class="attachment-container">
                    <div class="multiple-attachment">
                        <div class="w50 h100">
                            {first}
                        </div>
                        <div class="w50 h100">
                            {second}
                        </div>
                    </div>
                </div>
            }
            .into_any()
        }
        3 => {
            let first = attachments.remove(0);
            view! {
                <div class="attachment-container">
                    <div class="multiple-attachment">
                        <div class="primary w75 h100">
                            {first}
                        </div>
                        <div class="secondary w25 h50">
                            {attachments}
                        </div>
                    </div>
                </div>
            }
            .into_any()
        }
        4 => {
            let first = attachments.remove(0);
            let second = attachments.remove(0);
            let third = attachments.remove(0);
            let fourth = attachments.remove(0);
            view! {
                <div class="attachment-container">
                    <div class="multiple-attachment">
                        <div class="secondary w50 h50">
                            {first}
                            {second}
                        </div>
                        <div class="secondary w50 h50">
                            {third}
                            {fourth}
                        </div>
                    </div>
                </div>
            }
            .into_any()
        }
        _ => {
            let first = attachments.remove(0);
            view! {
                <div class="attachment-container">
                    <div class="multiple-attachment">
                        <div class="primary w75">
                            {first}
                        </div>
                        <div class="secondary w25">
                            {attachments}
                        </div>
                    </div>
                </div>
            }
            .into_any()
        }
    }
}

#[component]
pub fn TimelinePost(post: Status) -> impl IntoView {
    let source = post.clone();
    let post = post.enrich_content();

    let (post, reblogged_by) = match post.reblog {
        Some(inner_post) => (*inner_post, Some(post.account)),
        None => (post, None),
    };

    let attachments = match post.media_attachments {
        Some(attachments) => generate_attachments(attachments),
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
                <A href={ format!("/@/{}", account.acct) }>
                    <div class="inline">
                    <img src={ account.avatar.clone() } class="reblog-pfp pfp" />
                    <div class="no-decoration inline">
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
        <div class="post">
        <hr />
            {reblogged}
            <div class="user-link no-decoration">
                <A href={ format!("/@/{}", post.account.acct) }>
                    <div class="user-link inline no-decoration">
                        <img src={ post.account.avatar.clone() } class="timeline-pfp pfp" />
                        <div class="no-decoration">
                            <div class="inline">
                                { display_name }
                                {pronouns}
                            </div>
                            <p class="no-margin-recursive">{ format!("@{}", post.account.acct) }</p>
                        </div>
                    </div>
                </A>
            </div>
            {content}

            // <div class="status-actions">
            //     <button><MessageSquare/></button>
            //     <button><Repeat /></button>
            //     <button><Star /></button>
            //     <button><Bookmark /></button>
            //     <button><Share2 /></button>
            // </div>
            <RenderSrc src=serde_json::to_string_pretty(&source).unwrap() />
        <hr />
        </div>
    }
}
