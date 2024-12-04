use bayou_fe::{
    state::{Feed, State},
    timeline::feed::RenderFeed,
};
use leptos::mount::mount_to_body;
use leptos::prelude::ElementChild;
use leptos::{
    component,
    prelude::{provide_context, signal},
    view, IntoView, Params,
};
use leptos_router::components::*;
use leptos_router::params::Params;
use leptos_router::{
    components::{Router, Routes},
    hooks::use_params,
    path,
};

#[derive(Params, PartialEq, Clone)]
struct ProfileParams {
    id: Option<String>,
}

#[component]
pub fn profile() -> impl IntoView {
    let params = use_params::<ProfileParams>();
    // let id = move || params.read().clone().;
    todo!()
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
              <Route path=path!("/@/:id") view=public/>
              <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> }/>
              </Routes>
        </main>
      </Router>
    }
}

fn main() {
    mount_to_body(App);
}
