use leptos::prelude::*;

use crate::api::types::api_message::ApiMessage;

#[component]
pub fn Message(message: ApiMessage, render_user: bool) -> impl IntoView {
    let display_name = message.user.display_name.unwrap_or(message.user.username);
    let user = view! {
        <img src="https://picsum.photos/64" />
        <h2>{display_name}</h2>
    };
    let topper = match message.is_reply {
        true => view! {
            <p>"in reply"</p>
        }.into_any(),
        false => view! {}.into_any(),
    };
    view! {

    }
}