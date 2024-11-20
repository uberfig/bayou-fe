use leptos::{component, html::div, view, IntoView};
use leptos_lucide_icons::{Bookmark, MessageSquare, Repeat, Share2, Star};

use crate::masto_types::timeline_item::Status;

#[component]
pub fn TimelinePost(post: Status) -> impl IntoView {
    let content = div().inner_html(post.content).attr("class", "post-body");
    let content = match post.sensitive {
        true => {
            match post.spoiler_text.is_empty() {
                true => content.into_view(),
                false => view! {
                    <details>
                        <summary>{ post.spoiler_text }</summary>
                        {content}
                    </details>
                }.into_view(),
            }
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
        <hr />
        </div>
    }
}
