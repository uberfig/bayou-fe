use leptos::{component, prelude::*, view, IntoView, Params};
use leptos_router::{hooks::use_params, params::Params};

#[derive(Params, PartialEq, Clone)]
struct ProfileParams {
    webfinger: Option<String>,
}

#[component]
pub fn Profile() -> impl IntoView {
    let params = use_params::<ProfileParams>();

    let webfinger = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.webfinger.clone())
            .unwrap_or("missing".to_string())
    };
    view! {<h1>{webfinger}</h1>}
}
