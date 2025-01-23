use leptos::{
    component,
    prelude::{ElementChild, For, Get, IntoAny, ReadSignal, StorageAccess},
    server::LocalResource,
    view, IntoView,
};

use crate::{api::masto_types::status::Status, components::status::post::TimelinePost};

#[component]
pub fn TimelineSegment(posts: Vec<(Status, Option<Vec<Status>>)>) -> impl IntoView {
    view! {
        <For
            each=move || posts.clone()
            key=|post| post.0.id.clone()
            children=move |post: (Status, Option<Vec<Status>>)| {
                let (post, chain) = post;
                view! {
                  <TimelinePost post=post with_link=true reply_chain=chain render_chain=true/>
                  <hr />
                }
              }
        />
    }
}

#[derive(Clone)]
pub struct Segment {
    pub contents: LocalResource<Vec<(Status, Option<Vec<Status>>)>>,
    pub id: String,
}

#[component]
pub fn SegmentWrap(segment: Segment) -> impl IntoView {
    view! {
        {move || match segment.contents.get() {
            None => view! { <p>"Loading..."</p> }.into_any(),
            Some(data) => view! { <TimelineSegment posts=data.into_taken()/> }.into_any()
        }}
    }
}

#[component]
pub fn SegmentList(segments: ReadSignal<Vec<Segment>>) -> impl IntoView {
    view! {
        <For
            each=move || segments.get()
            key=|segment| segment.id.clone()
            children=move |segment: Segment| {
                view! {
                  <SegmentWrap segment=segment/>
                }
              }
        />
    }
}
