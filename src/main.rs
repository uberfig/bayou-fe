use bayou_fe::{
    api::{
        methods::auth::register::register_device,
        types::devices::{device_info::DeviceInfo, registered_device::RegisteredDevice},
    }, components::registering::Registering, routes::app_routes::AppRoutes, state::{State, DEVICE_TOKEN}
};
use leptos::{prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_use::storage::use_local_storage;

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
