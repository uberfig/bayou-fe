use leptos::{component, html::div, view, IntoView, View};
use leptos_lucide_icons::{Bookmark, MessageSquare, Repeat, Share2, Star};

use crate::masto_types::timeline_item::{MediaAttachment, Status};

pub fn generate_attachments(attachments: Vec<MediaAttachment>) -> View {
    let mut attachments = attachments.into_iter()
            .map(|attachment| {
                match attachment.type_field {
                    crate::masto_types::timeline_item::MediaType::Unknown => view! {<a href={attachment.url.to_string()}>{attachment.url.to_string()}</a>}.into_view(),
                    crate::masto_types::timeline_item::MediaType::Image => view! {
                        <img class="attachment attachment-img" src={attachment.url.to_string()} alt={attachment.description}/>
                    }.into_view(),
                    crate::masto_types::timeline_item::MediaType::Gifv => view! {<a href={attachment.url.to_string()}>{attachment.url.to_string()}</a>}.into_view(),
                    crate::masto_types::timeline_item::MediaType::Video => view! {<p>{attachment.url.to_string()}</p>}.into_view(),
                    crate::masto_types::timeline_item::MediaType::Audio => view! {
                        <audio controls>
                            <source src={attachment.url.to_string()} type="audio" />
                            {"Your browser does not support the audio element."}
                        </audio> 
                    }.into_view(),
                }
            })
            .collect::<Vec<_>>();
    
    match attachments.len() {
        0 => view! {}.into_view(),
        1 => view! {
            <div class="attachment-container">
                {attachments}
            </div>
        }.into_view(),
        2 => {
            let first = attachments.remove(0);
            let second = attachments.remove(0);
            view! {
                <div class="attachment-container">
                    <div class="multiple-attachment">
                        <div class="w50">
                            {first}
                        </div>
                        <div class="w50">
                            {second}
                        </div>
                    </div>
                </div>
            }.into_view()
        }
        3 => {
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
            }.into_view()
        },
        4 =>  {

            let first = attachments.remove(0);
            let second = attachments.remove(0);
            let third = attachments.remove(0);
            let fourth = attachments.remove(0);
            view! {
                <div class="attachment-container">
                    <div class="multiple-attachment">
                        <div class="secondary w50">
                            {first}
                            {second}
                        </div>
                        <div class="secondary w50">
                            {third}
                            {fourth}
                        </div>
                    </div>
                </div>
            }.into_view()
        
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
            }.into_view()
        }
    }
}

#[component]
pub fn TimelinePost(post: Status) -> impl IntoView {
    let source = post.clone();

    let attachments = match post.media_attachments {
        Some(attachments) => {
            generate_attachments(attachments)
        }
        None => view! {}.into_view(),
    };

    let content = div().inner_html(post.content).attr("class", "post-body");
    let content = view! {
        {content}
        {attachments}
    };
    let content = match post.sensitive {
        true => match post.spoiler_text.is_empty() {
            true => content.into_view(),
            false => view! {
                <details>
                    <summary>{ post.spoiler_text }</summary>
                    {content}
                </details>
            }
            .into_view(),
        },
        false => content.into_view(),
    };

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
            <details>
                <summary>{ "source" }</summary>
                <pre>
                {serde_json::to_string_pretty(&source)}
                </pre>
            </details>
        <hr />
        </div>
    }
}
