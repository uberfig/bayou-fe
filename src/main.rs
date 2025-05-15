use bayou_fe::{
    api::{
        methods::auth::register::register_device,
        types::{
            auth_token::AuthToken,
            devices::{device_info::DeviceInfo, registered_device::RegisteredDevice},
        },
    },
    routes::login::Login,
    state::{State, AUTH_TOKEN, DEVICE_TOKEN},
};
use leptos::{prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_router::{
    components::{Redirect, Route, Router, Routes},
    path,
};
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

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
