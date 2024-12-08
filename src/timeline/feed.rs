use leptos::prelude::ElementChild;
use leptos::{
    component,
    prelude::{signal, use_context, ClassAttribute, GetUntracked, ReadSignal},
    server::LocalResource,
    view, IntoView,
};

use crate::masto_api::timelines::{fetch_posts, TimelineParams};
use crate::{
    state::State,
    timeline::{
        loader::{FeedPos, LoadOlder},
        segments::{Segment, SegmentList},
    },
};

#[component]
pub fn RenderFeed(feed: String) -> impl IntoView {
    let (feed_pos, set_feed_pos) = signal(FeedPos {
        oldest_id: None,
        end_of_feed: false,
    });

    let state: ReadSignal<State> = use_context().expect("missing state");

    let curr_state = state.get_untracked();
    let first_link = TimelineParams::new(&curr_state).get_timeline_link(&curr_state, &feed);
    let cloned = first_link.clone();
    let first_segment = Segment {
        contents: LocalResource::new(move || fetch_posts(cloned.clone(), set_feed_pos)),
        id: first_link,
    };
    let (segments, set_segments) = signal(vec![first_segment]);

    view! {
        <div class="timeline">
            <SegmentList segments=segments/>
            <LoadOlder
                feed_state=feed_pos
                set_feed_state=set_feed_pos
                state=state
                segments=set_segments
                feed=feed
            />
        </div>
    }
}
