use bayou_fe::{
    api::{
        methods::auth::{login::login, register::register_device},
        types::{
            auth_token::{AuthToken, DBAuthToken},
            devices::{device_info::DeviceInfo, registered_device::RegisteredDevice},
            login_err::LoginErr,
            login_request::LoginRequest,
        },
    },
    state::{PersistantState, State},
};
use leptos::{prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_router::{
    components::{Redirect, Route, Router, Routes},
    path,
};
use leptos_use::storage::use_local_storage;

const AUTH_TOKEN: &str = "auth-token";
const DEVICE_TOKEN: &str = "device-token";

#[component]
pub fn NotFinished() -> impl IntoView {
    view! {
        <p>
        "sorry this route is in progress"
        </p>
    }
}

#[component]
pub fn Registering() -> impl IntoView {
    view! {
        <p>
        "registering device..."
        </p>
    }
}

#[component]
pub fn LoginProtect<View: IntoView + Clone>(view: View) -> impl IntoView {
    let (logged_in, _, _) = use_local_storage::<Option<AuthToken>, JsonSerdeCodec>(AUTH_TOKEN);
    if logged_in.get_untracked().is_none() {
        return view! {<Redirect path="/login"/>}.into_any();
    }
    view.into_any()
}

#[component]
pub fn AppRoutes() -> impl IntoView {
    view! {
        <Routes fallback=|| "Not found.">
            <Route path=path!("/") view=|| view! {<LoginProtect view=NotFinished />}/>
            <Route path=path!("/login") view=Login/>
            <Route path=path!("/signup") view=NotFinished/>
            <Route path=path!("/rooms/@me") view=|| view! {<LoginProtect view=NotFinished />}/>
            // direct messages
            <Route path=path!("/rooms/@me/:room_id") view=|| view! {<LoginProtect view=NotFinished />}/>
            // room in a community
            <Route path=path!("/rooms/:community_id/:room_id") view=|| view! {<LoginProtect view=NotFinished />}/>
        </Routes>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (reg_device, set_reg_device, _) =
        use_local_storage::<Option<RegisteredDevice>, JsonSerdeCodec>(DEVICE_TOKEN);
    let state = State {
        prefix: "http://127.0.0.1:8020".to_string(),
    };
    let (state, _set_state) = signal(state);
    provide_context(state);

    let registering = match reg_device.get_untracked().is_none() {
        true => Some(LocalResource::new(move || {
            let state = state.get_untracked();
            async move {
                let device = register_device(&state, DeviceInfo::new()).await;
                if let Ok(device) = device.clone() {
                    set_reg_device.set(Some(device));
                };
                device
            }
        })),
        false => None,
    };

    let load_selector = move || {
        if let Some(reg) = registering {
            if reg.get().is_none() {
                return view! {<Registering/>}.into_any();
            }
        }
        view! {<AppRoutes/>}.into_any()
    };

    view! {
      <Router>
        <nav>
          /* ... */
        </nav>
        <main>
            {load_selector}
        </main>
      </Router>
    }
}
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
fn Login() -> impl IntoView {
    let (name, set_name) = signal("".to_string());
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
        <form>
            <label>
                "username:"
                <input type="username"
                    on:input:target=move |ev| {
                        let val: String = ev.target().value();
                        if !val.chars().all(|x| char::is_alphanumeric(x) || x.eq(&'_')) {
                            may_only_contain.set(true);
                            ev.target().set_value(&name.get());
                            return;
                        }
                        may_only_contain.set(false);
                        set_name.set(ev.target().value().to_lowercase());
                        ev.target().set_value(&name.get());
                    }
                    prop:value=name
                    disabled=move || loading.get()
                />
            </label>
            <label>
                "password:"
                <input type="password"
                    bind:value=password
                    disabled=move || loading.get()
                />
            </label>
            <input type="submit" value="Login"
                on:click=move |ev| {
                    ev.prevent_default();
                    loading.set(true);
                    login_result.set(Some(LocalResource::new(move || {
                        let device_id = reg_device.get_untracked()
                            .expect("should not be able to view login with a device that has not been registered")
                            .device_id;
                        let request = LoginRequest {
                            username: name.get_untracked(),
                            password: password.get_untracked(),
                            device_id,
                        };
                        login_tasks(request.to_owned(), state.get_untracked(), loading, set_reg_device, set_logged_in)
                    })));

                }
                disabled=move || loading.get()
            />
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
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
