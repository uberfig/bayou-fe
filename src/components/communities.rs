use leptos::{leptos_dom::logging::console_log, prelude::*};
use leptos_router::components::Redirect;
use crate::{
    api::{methods::communities::get_communities::joined_communites, types::auth_token::AuthToken}, routes::auth_routes::{AUTH_PREFIX, HOME_PREFIX}, state::State
};

#[component]
pub fn CommunitiesBar(refresh: RwSignal<()>, create_modal: RwSignal<bool>) -> impl IntoView {
    let logged_in = use_context::<Signal<Option<AuthToken>>>().expect("token should be provided");
    let state = use_context::<ReadSignal<State>>().expect("state should be provided");

    let resource = move || {
        LocalResource::new(move || {
            let state = state.get_untracked();
            let token = logged_in
                .get_untracked();
            joined_communites(state, token.unwrap_or_default())
        })
    };

    let loaded = RwSignal::new(resource());

    let mut first_time = true;

    Effect::new(move || {
        refresh.get();
        match first_time {
            true => {
                first_time = false;
            }
            false => {
                console_log("refreshing comms");
                loaded.set(resource());
            }
        }
    });

    let render = move || {
        let loaded = loaded.get();
        view! {
            <div class="comm_bar">
                <ul>
                    <li><a href=HOME_PREFIX>"home"</a></li>
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
                                        <li><a href={format!("{}/{}", AUTH_PREFIX, c.id)}>{c.get_abbrv()}</a></li>
                                    })
                                    .collect::<Vec<_>>()
                                    .into_any()
                            },
                            Some(Err(err)) => {
                                match err {
                                    crate::api::methods::request_err::RequestErr::Unauthorized => {
                                        view! {<Redirect path="/login"/>}.into_any()
                                    },
                                    crate::api::methods::request_err::RequestErr::Unkown => todo!(),
                                }
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
