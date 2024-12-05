use bayou_fe::{
    state::{Feed, State},
    timeline::feed::RenderFeed,
};
use leptos::prelude::ElementChild;
use leptos::{
    component,
    prelude::{provide_context, signal},
    view, IntoView, Params,
};
use leptos::{mount::mount_to_body, prelude::Read};
use leptos_router::components::*;
use leptos_router::params::Params;
use leptos_router::{
    components::{Router, Routes},
    hooks::use_params,
    path,
};

#[derive(Params, PartialEq, Clone)]
struct ProfileParams {
    webfinger: Option<String>,
}

#[component]
pub fn Profile() -> impl IntoView {
    let params = use_params::<ProfileParams>();

    let webfinger = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.webfinger.clone())
            .unwrap_or("missing".to_string())
    };
    view! {<h1>{webfinger}</h1>}
}

#[component]
fn App() -> impl IntoView {
    let (state, _set_state) = signal(State {
        domain: "mastodon.social".to_string(),
        limit: 20,
    });
    provide_context(state);

    let public = || {
        view! {
            <RenderFeed
                feed=Feed::Public
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
          <Routes fallback=|| "Not found.">
              <Route path=path!("/") view=public/>
              <Route path=path!("/@/:webfinger") view=Profile/>
              <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> }/>
              </Routes>
        </main>
      </Router>
    }
}

fn main() {
    mount_to_body(App);
}
