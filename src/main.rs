use bayou_fe::{
    state::State,
    timeline::{
        loader::{fetch_posts, FeedPos, LoadOlder},
        segments::{Segment, SegmentList},
    },
};
use leptos::{
    component, create_signal, view, IntoView, SignalGet,
};

#[component]
fn App() -> impl IntoView {
    let (feed_pos, set_feed_pos) = create_signal(FeedPos {
        oldest_id: None,
        end_of_feed: false,
    });
    let (state, _set_state) = create_signal(State {
        domain: "mastodon.social".to_string(),
        limit: 20,
    });

    let first_link = state.get().get_timeline();
    let first_segment = Segment {
        contents: fetch_posts(first_link.clone(), set_feed_pos),
        id: first_link,
    };
    let (segments, set_segments) = create_signal(vec![first_segment]);

    view! {
        <SegmentList segments=segments/>
        <LoadOlder 
        feed_state=feed_pos
        set_feed_state=set_feed_pos
        state=state
        segments=set_segments
        />
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
