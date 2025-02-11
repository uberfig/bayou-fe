use bayou_fe::api::masto_api::timelines::{TimelineParams, PUBLIC_TIMELINE};
use bayou_fe::api::oauth::application::RegisterApplication;
use bayou_fe::api::oauth::scopes::Scopes;
use bayou_fe::components::status::status_page::StatusPage;
use bayou_fe::components::timeline::feed::RenderFeed;
use bayou_fe::components::user_profile::profile::Profile;
use bayou_fe::not_found::NotFound;
use bayou_fe::state::State;
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
        show_src: true,
        use_timeline_reply_chains: true,
        reply_chain_depth: 4,
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
    let register = RegisterApplication {
        client_name: "Bayou".to_string(),
        redirect_uris: vec!["http://127.0.0.1:8080/callback".to_string(), "http://127.0.0.1:8080/register".to_string()],
        scopes: Scopes::default().set_read(true).set_write(true).set_push(true),
        website: "joinbayou.org/app".to_string(),
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
