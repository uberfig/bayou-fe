use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use uuid::Uuid;

use crate::routes::auth_routes::AUTH_PREFIX;
use crate::{
    api::{methods::communities::get_comm_rooms::community_rooms, types::auth_token::AuthToken},
    state::State,
};

#[component]
pub fn CommunityRoomsBar(id: Uuid, refresh: RwSignal<()>, create_modal: RwSignal<bool>, room_count: RwSignal<usize>) -> impl IntoView {
    let logged_in = use_context::<ReadSignal<AuthToken>>().expect("token should be provided");
    let state = use_context::<ReadSignal<State>>().expect("state should be provided");

    let resource = move || {
        LocalResource::new(move || {
            let token = logged_in
                .get_untracked();
            let state = state.get_untracked();
            let community = id;
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
                    room_count.set(rooms.len());
                    let rooms = rooms.into_iter()
                    .map(|x| view! {
                        <li><a href={format!("{}/{}/{}", AUTH_PREFIX, id, x.id)}>{x.info.name}</a></li>
                    })
                    .collect::<Vec<_>>();
                    view! {
                        <ul>
                            <li>
                                <button
                                    on:click= move |_| {
                                        create_modal.set(true);
                                    }
                                >
                                    "new"
                                </button>
                            </li>
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
