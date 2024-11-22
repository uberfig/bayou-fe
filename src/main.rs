use bayou_fe::{
    state::{Feed, State},
    timeline::feed::RenderFeed,
};
use leptos::{component, create_signal, provide_context, view, IntoView};
use leptos_router::{Route, RouteProps, Router, RouterProps, Routes, RoutesProps};

#[component]
fn App() -> impl IntoView {
    let (state, _set_state) = create_signal(State {
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
                <Routes>
                <Route path="/" view=public/>
                <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
