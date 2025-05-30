use bayou_fe::{
    api::{
        methods::auth::register::register_device,
        types::devices::{device_info::DeviceInfo, registered_device::RegisteredDevice},
    },
    components::{
        comm::comm_routes::CommRoutes,
        login_protect::LoginProtect,
        personal::personal_routes::PersonalRoutes,
        registering::Registering,
    },
    routes::{login::Login, signup::Signup},
    state::{State, DEVICE_TOKEN},
};
use leptos::{prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_router::{
    components::{Outlet, ParentRoute, Redirect, Route, Router, Routes},
    path, MatchNestedRoutes,
};
use leptos_use::storage::use_local_storage;

#[component(transparent)]
fn RoomRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
      <ParentRoute path=path!("/rooms") view=Outlet >
        <Route path=path!("") view=|| view! { <LoginProtect view=|| view! {<Redirect path="/rooms/@me"/>} /> } />
        <PersonalRoutes />
        <CommRoutes />
      </ParentRoute>
    }
    .into_inner()
}

#[component]
pub fn AppRoutes() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=|| view! {<LoginProtect view=|| view! {<Redirect path="/rooms/@me"/>} />}/>
                <Route path=path!("/login") view=Login/>
                <Route path=path!("/signup") view=Signup/>
                <RoomRoutes />
            </Routes>
        </Router>
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
        {load_selector}
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
