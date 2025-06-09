use crate::components::{
    communities::CommunitiesBar,
    login_protect::LoginProtect,
    modal::{base_modal::BaseModal, create_comm::CreateComm},
};
use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Redirect, Route},
    path, MatchNestedRoutes,
};

pub const HOME_PREFIX: &str = "/app/@home";
pub const AUTH_PREFIX: &str = "/app";

use super::{comm_routes::CommRoutes, personal_routes::PersonalRoutes};

#[component]
pub fn AuthRoutesContainter() -> impl IntoView {
    let refresh = RwSignal::new(());
    let create_comm_modal = RwSignal::new(false);
    view! {
        <nav>
            <BaseModal view=move || view! {<CreateComm display=create_comm_modal refresh=refresh />}
            display=create_comm_modal />
            <CommunitiesBar refresh=refresh create_modal=create_comm_modal />
            <p>"above me are the joined communities"</p>
        </nav>
        <Outlet/>
    }
}

#[component(transparent)]
pub fn AuthRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
      <ParentRoute path=path!("/app") view=|| view! {<LoginProtect view=AuthRoutesContainter />} >
        <Route path=path!("") view=|| view! {<Redirect path=HOME_PREFIX/>} />
        <PersonalRoutes />
        <CommRoutes />
      </ParentRoute>
    }
    .into_inner()
}
