use bayou_fe::{api::{methods::auth::register::register_device, types::{auth_token::AuthToken, devices::{device_info::DeviceInfo, registered_device::RegisteredDevice}}}, state::{PersistantState, State}};
use leptos::{prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_router::{components::{Redirect, Route, Router, Routes}, path};
use leptos_use::storage::use_local_storage;

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
pub fn LoginProtect<View: IntoView+Clone>(view: View) -> impl IntoView {
    let (logged_in, _, _) = use_local_storage::<Option<AuthToken>, JsonSerdeCodec>("logged-in");
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
    let (reg_device, set_reg_device, _) = use_local_storage::<Option<RegisteredDevice>, JsonSerdeCodec>("reg-device");
    let state = State {
        prefix: "http://127.0.0.1:8020".to_string(),
    };

    let registering = match reg_device.get_untracked().is_none() {
        true => Some(LocalResource::new(move || {
            let state = state.clone();
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

#[component]
fn Login() -> impl IntoView {
    let (name, set_name) = signal("".to_string());
    let password = RwSignal::new("".to_string());
    let uname_pass_incorrect = RwSignal::new(false);
    let may_only_contain = RwSignal::new(false);
    let loading = RwSignal::new(false);

    view! {
        <form
            // on:submit=move |_| {
            //     may_only_contain.set(true);
            //     // ev.prevent_default();
            // }
        >
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
            <label>
                "Please send me lots of spam email."
                <input type="checkbox"
                    bind:checked=uname_pass_incorrect
                />
            </label>
            <input type="submit" value="Login"
                on:click=move |ev| {
                    uname_pass_incorrect.set(true);
                    ev.prevent_default();
                    loading.set(true);
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
        <p>{name}</p>
        
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
