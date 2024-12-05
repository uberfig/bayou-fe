use bayou_fe::{
    state::{Feed, State},
    timeline::feed::RenderFeed,
    user_profile::profile::Profile,
};
use leptos::mount::mount_to_body;
use leptos::prelude::ElementChild;
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
