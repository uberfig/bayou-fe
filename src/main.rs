use bayou_fe::{
    api::{
        methods::auth::register::register_device,
        types::devices::{device_info::DeviceInfo, registered_device::RegisteredDevice},
    },
    components::{
        communities::CommunitiesBar, login_protect::LoginProtect, registering::Registering,
        unimplimented::NotFinished,
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

#[component]
pub fn MainContainer() -> impl IntoView {
    view! {
        <nav>
            <CommunitiesBar />
        </nav>
        <main>
            <p>"above me are the joined communities and below the current room content"</p>
            <Outlet/>
        </main>
    }
}

#[component(transparent)]
fn RoomRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
      <ParentRoute path=path!("/rooms") view=MainContainer >
        <Route path=path!("") view=|| view! { <LoginProtect view=|| view! {<Redirect path="/rooms/@me"/>} /> } />
        <Route path=path!("/@me") view=|| view! {<LoginProtect view=|| view! { <p>"should display the dms on this page but none open, maybe have a pic of a logo/mascot here on pc"</p> } />}/>
        // direct messages
        <Route path=path!("/@me/:room_id") view=|| view! {<LoginProtect view=NotFinished />}/>
        <Route path=path!("/:community_id") view=|| view! {<LoginProtect view=NotFinished />}/>
        // room in a community
        <Route path=path!("/:community_id/:room_id") view=|| view! {<LoginProtect view=NotFinished />}/>
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
