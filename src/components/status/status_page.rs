use leptos::{
    component,
    prelude::*,
    view, IntoView, Params,
};
use leptos_router::{hooks::use_params, params::Params};

use crate::{
    api::masto_api::statuses::{request_status, status_request_link}, components::status::post::TimelinePost, not_found::NotFound, state::State,
    api::masto_types::status::Status
};

#[derive(Params, PartialEq, Clone)]
struct StatusParams {
    id: Option<String>,
}

struct ValidStatusParams {
    id: String,
}

impl StatusParams {
    fn validate(self) -> Option<ValidStatusParams> {
        match self.id {
            Some(id) => Some(ValidStatusParams { id }),
            None => None,
        }
    }
}

#[component]
pub fn StatusPage() -> impl IntoView {
    let params = use_params::<StatusParams>();

    let params = move || {
        params
            .read()
            .as_ref()
            .ok()
            .map(|x| x.to_owned().validate())
            .flatten()
    };

    let state: ReadSignal<State> = use_context().expect("missing state");
    let link = move || status_request_link(&state.get(), &params().unwrap().id);
    let (status, _set_status) = signal(LocalResource::new(move || request_status(link())));

    view! { <StatusWrap status=status.get()/> }
}

#[component]
pub fn StatusWrap(status: LocalResource<Option<Status>>) -> impl IntoView {
    view! {
        {move || match status.get() {
            None => view! { <p>"Loading..."</p> }.into_any(),
            Some(status) => match status.into_taken() {
                None => view! { <NotFound /> }.into_any(),
                Some(status) => view! { <Status status=status/> }.into_any(),
            }
        }}
    }
}

#[component]
pub fn Status(status: Status) -> impl IntoView {
    view! {<TimelinePost post=status with_link=false reply_chain=None render_chain=true/>}
}
