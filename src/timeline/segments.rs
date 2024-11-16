use leptos::{component, view, For, IntoView, Resource, SignalGet};

use crate::{masto_types::timeline_item::Post, timeline::post::TimelinePost};

#[component]
pub fn TimelineSegment(posts: Vec<Post>) -> impl IntoView {
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

pub struct Segment {
    pub contents: Resource<(), Vec<Post>>,
    pub id: String,
}

#[component]
pub fn SegmentWrap(segment: Segment) -> impl IntoView {
    view! {
        <div class="timeline">
        {move || match segment.contents.get() {
            None => view! { <p>"Loading..."</p> }.into_view(),
            Some(data) => view! { <TimelineSegment posts=data/> }.into_view()
        }}
        </div>
    }
}