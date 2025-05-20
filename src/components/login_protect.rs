use leptos::{prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_router::components::Redirect;
use leptos_use::storage::use_local_storage;

use crate::{api::types::auth_token::AuthToken, state::AUTH_TOKEN};

#[component]
pub fn LoginProtect<View: IntoView + Clone>(view: View) -> impl IntoView {
    let (logged_in, _, _) = use_local_storage::<Option<AuthToken>, JsonSerdeCodec>(AUTH_TOKEN);
    if logged_in.get_untracked().is_none() {
        return view! {<Redirect path="/login"/>}.into_any();
    }
    view.into_any()
}