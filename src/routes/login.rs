use leptos::{prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_use::storage::use_local_storage;

use crate::{
    api::{
        methods::auth::login::login,
        types::{
            auth_token::{AuthToken, DBAuthToken},
            devices::registered_device::RegisteredDevice,
            login_err::LoginErr,
            login_request::LoginRequest,
        },
    },
    components::username::UsernameEntry,
    state::{State, AUTH_TOKEN, DEVICE_TOKEN},
};

async fn login_tasks(
    request: LoginRequest,
    state: State,
    loading: RwSignal<bool>,
    set_reg: WriteSignal<Option<RegisteredDevice>>,
    set_logged_in: WriteSignal<Option<AuthToken>>,
) -> Result<DBAuthToken, LoginErr> {
    let result = login(&state, &request).await;
    if matches!(result, Err(LoginErr::InvalidDevice)) {
        set_reg.set(None);
    }
    loading.set(false);
    if let Ok(token) = result.clone() {
        set_logged_in.set(Some(token.required_token));
    }
    result
}

#[component]
pub fn Login() -> impl IntoView {
    let username = RwSignal::new("".to_string());
    let password = RwSignal::new("".to_string());
    let uname_pass_incorrect = RwSignal::new(false);
    let may_only_contain = RwSignal::new(false);

    let loading = RwSignal::new(false);
    let login_result: RwSignal<Option<LocalResource<Result<DBAuthToken, LoginErr>>>> =
        RwSignal::new(None);

    let (logged_in, set_logged_in, _) =
        use_local_storage::<Option<AuthToken>, JsonSerdeCodec>(AUTH_TOKEN);
    let (reg_device, set_reg_device, _) =
        use_local_storage::<Option<RegisteredDevice>, JsonSerdeCodec>(DEVICE_TOKEN);
    let state = use_context::<ReadSignal<State>>().expect("state should be provided");

    view! {
        <main>
        <form>
            <p>
                <label for="username">"username:"</label>
                <UsernameEntry
                    username=username
                    may_only_contain=may_only_contain
                    disabled=loading
                />
            </p>
            <p>
            <label for="password">"password:"</label>
                <input type="password" id="password"
                    bind:value=password
                    disabled=move || loading.get()
                    required
                />

            </p>
            <p>
                <button type="submit"
                    on:click=move |ev| {
                        ev.prevent_default();
                        loading.set(true);
                        login_result.set(Some(LocalResource::new(move || {
                            let device_id = reg_device.get_untracked()
                                .expect("should not be able to view login with a device that has not been registered")
                                .device_id;
                            let request = LoginRequest {
                                username: username.get_untracked(),
                                password: password.get_untracked(),
                                device_id,
                            };
                            login_tasks(request, state.get_untracked(), loading, set_reg_device, set_logged_in)
                        })));

                    }
                    disabled=move || loading.get()
                >
                "Login"
                </button>
            </p>
        </form>
        <Show when=move || uname_pass_incorrect.get()>
            <p>"username or password is incorrect"</p>
        </Show>
        <Show when=move || may_only_contain.get()>
            <p>"username may only contain numbers, letters, and underscores"</p>
        </Show>
        <Show when=move || loading.get()>
            <p>"loading..."</p>
        </Show>

        <Show when=move || {
            if let Some(val) = login_result.get() {
                if let Some(val) = val.get() {
                    return val.is_err();
                }
            }
            false
        }>
            <p>
                {move || {
                    if let Some(val) = login_result.get() {
                        if let Some(val) = val.get() {
                            if let Err(reason) = val {
                                return reason.to_string();
                            }
                        }
                    }
                    "an unexpected error has occured".to_string()
                }}
            </p>
        </Show>

        <Show when=move || logged_in.get().is_some()>
            <button
                on:click=move |_| {
                    set_logged_in.set(None);
                }
            >
                "logout"
            </button>
        </Show>
        </main>
    }
}
