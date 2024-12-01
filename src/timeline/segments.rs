use leptos::{component, view, For, IntoView, ReadSignal, Resource, SignalGet};

use crate::{masto_types::timeline_item::Status, timeline::post::TimelinePost};

#[component]
pub fn TimelineSegment(posts: Vec<Status>) -> impl IntoView {
    view! {
        <For
            each=move || posts.clone()
            key=|post| post.id.clone()
            children=move |post: Status| {
                view! {
                  <TimelinePost post=post/>
                }
              }
        />
    }
}

#[derive(Clone)]
pub struct Segment {
    pub contents: Resource<(), Vec<Status>>,
    pub id: String,
}

#[component]
pub fn SegmentWrap(segment: Segment) -> impl IntoView {
    view! {
        {move || match segment.contents.get() {
            None => view! { <p>"Loading..."</p> }.into_view(),
            Some(data) => view! { <TimelineSegment posts=data/> }.into_view()
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
