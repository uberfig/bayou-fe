use crate::{api::types::api_message::ApiMessage, components::room::message::Message};
use leptos::prelude::*;

#[derive(Clone)]
pub enum Segment {
    Loaded(LocalResource<Vec<ApiMessage>>),
    Live(Vec<ApiMessage>),
}

/// messages should be from oldest to newest
#[component]
pub fn MessageSegment(messages: Vec<ApiMessage>) -> impl IntoView {
    // reversed so its newest to oldest, next should be the older message
    let mut iter = messages.into_iter().rev().peekable();
    
    let mut render = Vec::new();
    while let Some(message) = iter.next() {
        let show_user = 'show: {
            if let Some(prior) = iter.peek() {
                if prior.user.id == message.user.id {
                    break 'show false;
                } 
            } 
            true
        };
        render.push(view! {
            <Message message=message render_user=show_user />
        });
    }
    let render = render.into_iter().rev().collect::<Vec<_>>();
    view! {
        {render}
    }
}

#[component]
pub fn SegmentWrap(segment: Segment) -> impl IntoView {
    let to_render = move || {
        match segment.to_owned() {
            Segment::Loaded(loader) => match loader.get() {
                Some(messages) => view! {<MessageSegment messages=messages/>}.into_any(),
                None => view! { <p>"Loading..."</p> }.into_any(),
            },
            Segment::Live(messages) => view! {<MessageSegment messages=messages/>}.into_any(),
        }
    };
    view! {{to_render}}
}

#[component]
pub fn SegmentList(segments: RwSignal<Vec<Segment>>) -> impl IntoView {
    let list = move || {
        segments.get()
            .into_iter()
            .map(|segment| view! {<SegmentWrap segment=segment/>})
            .collect::<Vec<_>>()
    };
    view! {{list}}
}
