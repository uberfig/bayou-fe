use leptos::{
    component, create_signal, use_context, view, IntoView, ReadSignal, SignalGetUntracked,
};

use crate::{
    state::{Feed, State},
    timeline::{
        loader::{fetch_posts, FeedPos, LoadOlder},
        segments::{Segment, SegmentList},
    },
};

#[component]
pub fn RenderFeed(feed: Feed) -> impl IntoView {
    let (feed_pos, set_feed_pos) = create_signal(FeedPos {
        oldest_id: None,
        end_of_feed: false,
    });

    let state: ReadSignal<State> = use_context().expect("missing state");

    let first_link = state.get_untracked().get_timeline_link(feed);
    let first_segment = Segment {
        contents: fetch_posts(first_link.clone(), set_feed_pos),
        id: first_link,
    };
    let (segments, set_segments) = create_signal(vec![first_segment]);

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
