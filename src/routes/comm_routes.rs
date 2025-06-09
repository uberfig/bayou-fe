use crate::components::{comm::comm_rooms_bar::CommunityRoomsBar, modal::{base_modal::BaseModal, create_room::CreateRoom}, room::room::Room};
use leptos::Params;
use leptos_router::{hooks::use_params, params::Params};
use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route},
    path, MatchNestedRoutes,
};
use uuid::Uuid;

#[derive(Params, PartialEq)]
pub struct CommId {
    pub community_id: Option<Uuid>,
}

#[component]
pub fn CommContainer() -> impl IntoView {
    let refresh_rooms = RwSignal::new(());
    let params = use_params::<CommId>();
    let create_room_modal = RwSignal::new(false);
    let room_count: RwSignal<usize> = RwSignal::new(0);

    let id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.community_id)
            .unwrap_or_default()
    };

    let get_view = move || {
        let id = id();

        view! {
            <BaseModal 
                view=move || view! {<CreateRoom display=create_room_modal refresh=refresh_rooms comm=id room_count=room_count />}
                display=create_room_modal 
            />
            <h1>"todo display comm name"</h1>
            <nav>
                <CommunityRoomsBar refresh=refresh_rooms id=id create_modal=create_room_modal room_count=room_count />
            </nav>
            <main>
                <Outlet/>
            </main>
        }
    };
    
    view! {{get_view}}
}

#[component(transparent)]
pub fn CommRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
      <ParentRoute path=path!("/:community_id") view=CommContainer >
        <Route path=path!("") view=|| view! { <p>"meow the community description goes here"</p> } />
        // should display rooms with the current highlighted
        <Route path=path!("/:room_id") view=|| view! { <h2>"todo display room name here"</h2> <Room /> }/>
      </ParentRoute>
    }
    .into_inner()
}
