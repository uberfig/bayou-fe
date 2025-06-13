use leptos::{prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_router::components::Redirect;
use leptos_use::storage::use_local_storage;

use crate::{api::types::auth_token::AuthToken, state::AUTH_TOKEN};

#[component]
pub fn LoginProtect<View: IntoView + Clone+ 'static>(view: View) -> impl IntoView {
    let (logged_in, _, _) = use_local_storage::<Option<AuthToken>, JsonSerdeCodec>(AUTH_TOKEN);
    let rendered = move || {
        match logged_in.get().is_none() {
            true => view! {<Redirect path="/login"/>}.into_any(),
            false => view.clone().into_any(),
        }
    };
    view! {{rendered}}
}
