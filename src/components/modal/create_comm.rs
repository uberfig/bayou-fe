use codee::string::JsonSerdeCodec;
use leptos::{leptos_dom::logging::console_log, prelude::*};
use leptos_router::components::Redirect;
use leptos_use::storage::use_local_storage;

use crate::{api::{methods::communities::create_comm::{create_comm, Communityinfo}, types::{api_community::ApiCommunity, auth_token::AuthToken}}, state::{State, AUTH_TOKEN}};

pub fn create<F>(
    state: State,
    auth: AuthToken,
    comm: Communityinfo,
    completed: F,
) -> LocalResource<Result<ApiCommunity, ()>>
where
    F: FnOnce(bool) + Clone + 'static,
{
    let loader = LocalResource::new(move || {
        let state = state.to_owned();
        let auth = auth.to_owned();
        let comm = comm.to_owned();
        let completed = completed.clone();
        async move {
            let result = create_comm(state, auth, comm).await;
            completed(result.is_ok());
            result
        }
    });

    loader
}

#[component]
pub fn CreateComm(display: RwSignal<bool>, refresh: RwSignal<()>) -> impl IntoView {
    let name = RwSignal::new("".to_string());
    let loading = RwSignal::new(false);
    let create_result: RwSignal<Option<LocalResource<Result<ApiCommunity, ()>>>> =
        RwSignal::new(None);

    let (auth, _, _) = use_local_storage::<Option<AuthToken>, JsonSerdeCodec>(AUTH_TOKEN);
    let state = use_context::<ReadSignal<State>>().expect("state should be provided");

    let completed = move |_| {
        loading.set(false);
        refresh.set(());
    };
    
    let create_clicked = move || {
        loading.set(true);
        let comm = Communityinfo { name: name.get_untracked(), description: None };
        create_result.set(Some(create(state.get_untracked(), auth.get_untracked().expect("not logged in"), comm, completed)));
    };

    view! {
        <form class="create_comm" autofocus>
        <p>
            <label for="name">"community name:"</label>
        </p>
        <p>
            <input type="text" id="name"
                bind:value=name
                required
                disabled=move || loading.get()
            />
            <button type="submit"
                on:click=move |ev| {
                    ev.prevent_default();
                    create_clicked();
                }
                disabled=move || loading.get()
            >
            "Create"
            </button>
            </p>
        </form>
        <button
            on:click=move |_| {
                display.set(false);
            }
        >
        "cancel"
        </button>
        <Show when=move || {
            if let Some(val) = create_result.get() {
                if let Some(val) = val.get() {
                    return val.is_ok();
                }
            }
            false
        }>
            {move || {
                if let Some(val) = create_result.get() {
                    if let Some(val) = val.get() {
                        if let Ok(completed) = val {
                            create_result.set(None);
                            display.set(false);
                            name.set("".to_string());
                            return view! {
                                <Redirect path=format!("/rooms/{}", completed.id.as_simple().to_string())/>
                            }.into_any();
                        }
                    }
                }
                view! {<p>"an unexpected error has occured".to_string()</p>}.into_any()
            }}
        </Show>
    }
}
