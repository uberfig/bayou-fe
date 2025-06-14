use leptos::prelude::*;

use crate::{api::types::api_message::ApiMessage, components::room::message_sender::MessageReply};

#[component]
pub fn Message(message: ApiMessage, render_user: bool, replying: RwSignal<Option<MessageReply>>) -> impl IntoView {
    let display_name = message.user.display_name.unwrap_or(message.user.username);
    let user = view! {
        // <img src="https://picsum.photos/64" />
        <h2>{display_name.clone()}</h2>
    };
    let user = match render_user {
        true => user.into_any(),
        false => view! {}.into_any(),
    };
    let topper = match message.is_reply {
        true => view! {
            <p>"in reply"</p>
        }
        .into_any(),
        false => view! {}.into_any(),
    };
    view! {
        <div>
            {topper}
            {user}
            <p>
                {message.content}
            </p>
            <button
                    on:click=move |_| {
                        replying.set(Some(MessageReply { display_name: display_name.clone(), message_id: message.id }));
                    }
                >
                "Reply"
            </button>
            // <p>
            // // "uuid: " {message.id.as_simple().to_string()}
            // </p>
        </div>
    }
}
