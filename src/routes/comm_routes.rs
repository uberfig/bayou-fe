use crate::components::{comm::comm_rooms_bar::CommunityRoomsBar, room::room::Room};
use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route},
    path, MatchNestedRoutes,
};

#[component]
pub fn CommContainer() -> impl IntoView {
    view! {
        <nav>
            <CommunityRoomsBar />
        </nav>
        <main>
            <Outlet/>
        </main>
    }
}

#[component(transparent)]
pub fn CommRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
      <ParentRoute path=path!("/:community_id") view=CommContainer >
        <Route path=path!("") view=|| view! { <p>"meow the community description goes here"</p> } />
        // should display rooms with the current highlighted
        <Route path=path!("/:room_id") view=|| view! { <p>"meow the community room goes here"</p> <Room /> }/>
      </ParentRoute>
    }
    .into_inner()
}
