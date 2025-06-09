use leptos::leptos_dom::logging::console_log;
use leptos::Params;
use leptos::{prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_router::{hooks::use_params, params::Params};
use leptos_use::storage::use_local_storage;
use uuid::Uuid;

use crate::{
    api::{methods::communities::get_comm_rooms::community_rooms, types::auth_token::AuthToken},
    state::{State, AUTH_TOKEN},
};

#[derive(Params, PartialEq)]
struct CommId {
    community_id: Option<Uuid>,
}

#[component]
pub fn CommunityRoomsBar(refresh: RwSignal<()>) -> impl IntoView {
    let params = use_params::<CommId>();
    let (logged_in, _, _) = use_local_storage::<Option<AuthToken>, JsonSerdeCodec>(AUTH_TOKEN);
    let state = use_context::<ReadSignal<State>>().expect("state should be provided");

    let id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.community_id)
            .unwrap_or_default()
    };
    let resource = move || {
        LocalResource::new(move || {
            let token = logged_in
                .get_untracked()
                .expect("trying to get comm rooms when not logged in");
            let state = state.get_untracked();
            let community = id();
            community_rooms(state, token, community)
        })
    };
    let (rooms, set_rooms) = signal(resource());
    let mut first_time = true;

    Effect::new(move || {
        refresh.get();
        match first_time {
            true => {
                first_time = false;
            }
            false => {
                console_log("refreshing rooms");
                set_rooms.set(resource());
            }
        }
    });

    let render_rooms =
        move || {
            let rooms = rooms.get();
            match rooms.get() {
                Some(Ok(rooms)) => {
                    let rooms = rooms.into_iter()
                    .map(|x| view! {
                        <li><a href={format!("/rooms/{}/{}", id(), x.id)}>{x.info.name}</a></li>
                    })
                    .collect::<Vec<_>>();
                    view! {
                        <ul>
                            {rooms}
                        </ul>
                    }
                    .into_any()
                }
                Some(Err(_)) => todo!(),
                None => view! {<p>"..."</p>}.into_any(),
            }
        };
    view! {
        {render_rooms}
    }
}
