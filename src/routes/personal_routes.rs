use crate::components::unimplimented::NotFinished;
use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route},
    path, MatchNestedRoutes,
};

#[component]
pub fn PersonalContainer() -> impl IntoView {
    view! {
        <nav>
            <p>"user dms go here"</p>
        </nav>
        <main>
            <Outlet/>
        </main>
    }
}

#[component(transparent)]
pub fn PersonalRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
      <ParentRoute path=path!("/@home") view=PersonalContainer >
        <Route path=path!("") view=|| view! { <p>"meow should display the dms on this page but none open, maybe have a pic of a logo/mascot here on pc"</p> } />
        // should display user dms with the current one highlighted in nav
        <Route path=path!("/:room_id") view=NotFinished/>
      </ParentRoute>
    }
    .into_inner()
}
