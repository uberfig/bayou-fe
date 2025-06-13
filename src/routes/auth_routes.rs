use crate::{api::types::{auth_token::AuthToken, socket_msg::SocketMsg}, components::{
    communities::CommunitiesBar,
    login_protect::LoginProtect,
    modal::{base_modal::BaseModal, create_comm::CreateComm},
}, state::AUTH_TOKEN};
use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Redirect, Route},
    path, MatchNestedRoutes,
};
use leptos_use::{core::ConnectionReadyState, storage::use_local_storage, use_websocket_with_options, ReconnectLimit, UseWebSocketOptions, UseWebSocketReturn};

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
    let (logged_in, set_logged_in, _) = use_local_storage::<Option<AuthToken>, JsonSerdeCodec>(AUTH_TOKEN);
    let UseWebSocketReturn {
        ready_state,
        message,
        send,
        // open,
        // close,
        ..
    } = use_websocket_with_options::<AuthToken, SocketMsg, JsonSerdeCodec, _, _>(
        "ws://127.0.0.1:8020/api/bayou_v1/ws",
        UseWebSocketOptions::default()
            .immediate(true)
            .reconnect_limit(ReconnectLimit::Infinite), // .on_open()
    );

    // let (logged_in, set_logged_in) = signal(logged_in_storage.get_untracked().expect("not logged in"));
    // Effect::new(move |_| {
    //     set_logged_in.set(logged_in_storage.get().expect("not logged in"));
    // });
    provide_context(logged_in);

    Effect::new(move |_| {
        if ConnectionReadyState::Open == ready_state.get() {
            send(&logged_in.get_untracked().unwrap_or_default());
        }
    });

    view! {
      <ParentRoute path=path!("/app") view=|| view! {<LoginProtect view=AuthRoutesContainter />} >
        <Route path=path!("") view=|| view! {<Redirect path=HOME_PREFIX/>} />
        <PersonalRoutes />
        <CommRoutes message=message />
      </ParentRoute>
    }
    .into_inner()
}
