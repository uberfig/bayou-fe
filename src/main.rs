use bayou_fe::masto_api::timelines::TimelineParams;
use bayou_fe::not_found::NotFound;
use bayou_fe::status::status_page::StatusPage;
use bayou_fe::{
    masto_api::timelines::PUBLIC_TIMELINE, state::State, timeline::feed::RenderFeed,
    user_profile::profile::Profile,
};
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos::{
    component,
    prelude::{provide_context, signal},
    view, IntoView,
};
use leptos_router::components::*;
use leptos_router::{
    components::{Router, Routes},
    path,
};

#[component]
fn App() -> impl IntoView {
    let (state, _set_state) = signal(State {
        domain: "mastodon.social".to_string(),
        limit: 20,
    });
    provide_context(state);

    let public = move || {
        view! {
            <RenderFeed
                feed=PUBLIC_TIMELINE.to_string()
                params=TimelineParams::new(&state.get_untracked())
            />
        }
    };

    view! {
        <Router>
            <nav>
            /* ... */
            </nav>
            <main>
            // all our routes will appear inside <main>
            <Routes fallback=|| view! { <NotFound /> }>
                <Route path=path!("/") view=public/>
                <Route path=path!("/:webfinger") view=Profile />
                <Route path=path!("/notes/:id") view=StatusPage />
                <Route path=path!("/*any") view=|| view! { <NotFound /> }/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
