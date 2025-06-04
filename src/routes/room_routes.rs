use crate::components::{communities::CommunitiesBar, login_protect::LoginProtect};
use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Redirect, Route},
    path, MatchNestedRoutes,
};

use super::{comm_routes::CommRoutes, personal_routes::PersonalRoutes};

#[component]
pub fn AuthRoutesContainter() -> impl IntoView {
    view! {
        <nav>
            <CommunitiesBar />
            <p>"above me are the joined communities"</p>
        </nav>
        <Outlet/>
    }
}

#[component(transparent)]
pub fn RoomRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
      <ParentRoute path=path!("/rooms") view=|| view! {<LoginProtect view=AuthRoutesContainter />} >
        <Route path=path!("") view=|| view! {<Redirect path="/rooms/@home"/>} />
        <PersonalRoutes />
        <CommRoutes />
      </ParentRoute>
    }
    .into_inner()
}
