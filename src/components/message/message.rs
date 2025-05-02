use leptos::prelude::*;

use crate::api::types::api_message::ApiMessage;

#[component]
fn Message(_message: ApiMessage) -> impl IntoView {
    view! {
        <p>
            {_message.user.display_name} ": "
        </p>
        <p>
            {_message.content}
        </p>
    }
}
