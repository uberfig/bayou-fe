use leptos::{leptos_dom::logging::console_log, prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_use::storage::use_local_storage;

use crate::{
    api::{methods::communities::get_communities::joined_communites, types::auth_token::AuthToken},
    state::{State, AUTH_TOKEN},
};

#[component]
pub fn CommunitiesBar(refresh: RwSignal<()>, create_modal: RwSignal<bool>) -> impl IntoView {
    let (logged_in, _, _) = use_local_storage::<Option<AuthToken>, JsonSerdeCodec>(AUTH_TOKEN);
    let state = use_context::<ReadSignal<State>>().expect("state should be provided");

    let resource = move || {
        LocalResource::new(move || {
            let state = state.get_untracked();
            let token = logged_in
                .get_untracked()
                .expect("communities bar visable when not logged in");
            joined_communites(state, token)
        })
    };

    let loaded = RwSignal::new(resource());

    let mut first_time = true;
    
    Effect::new(move || {
        refresh.get();
        match first_time {
            true => {
                first_time=false;
            },
            false => {
                console_log("refreshing comms");
                loaded.set(resource());
            },
        }
    });

    let render = move || {
        let loaded = loaded.get();
        view! {
            <div class="comm_bar">
                <ul>
                    <li><a href="/rooms/@home">"home"</a></li>
                    <li>
                        <button 
                            on:click= move |_| {
                                create_modal.set(true);
                            }
                        >
                            "new"
                        </button>
                    </li>
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
    };
    view! {{render}}
}
