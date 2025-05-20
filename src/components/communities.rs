use leptos::{prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_use::storage::use_local_storage;

use crate::{
    api::{methods::communities::get_communities::joined_communites, types::auth_token::AuthToken},
    state::{State, AUTH_TOKEN},
};

#[component]
pub fn CommunitiesBar() -> impl IntoView {
    let (logged_in, _, _) = use_local_storage::<Option<AuthToken>, JsonSerdeCodec>(AUTH_TOKEN);
    let state = use_context::<ReadSignal<State>>().expect("state should be provided");
    let loaded = LocalResource::new(move || {
        let state = state.get_untracked();
        let token = logged_in
            .get_untracked()
            .expect("communities bar visable when not logged in");
        joined_communites(state, token)
    });

    view! {
        <div class="comm_bar">
            <ul>
                {move || {
                    match loaded.get() {
                        Some(Ok(comms)) => {
                            comms.into_iter()
                                .map(|c| view! {
                                    <li><a href={format!("/rooms/{}", c.id)}>{c.get_abbrv()}</a></li>
                                })
                                .collect::<Vec<_>>()
                                .into_any()
                        },
                        Some(Err(_)) => {
                            todo!()
                        }
                        None => view! {
                            <p>"..."</p>
                        }.into_any(),
                    }
                }}
            </ul>
        </div>
    }
}
