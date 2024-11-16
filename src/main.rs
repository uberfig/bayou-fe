use bayou_fe::{
    state::{Feed, State}, timeline::feed::RenderFeed}
;
use leptos::{
    component, create_signal, view, IntoView
};

#[component]
fn App() -> impl IntoView {
    let (state, _set_state) = create_signal(State {
        domain: "mastodon.social".to_string(),
        limit: 20,
    });

    view! {
        <RenderFeed 
        state=state
        feed=Feed::Public
        />
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
