use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};
use uuid::Uuid;

use crate::components::room::{chat_log::ChatLog, message_sender::{MessageInput, MessageReply}};


#[derive(Params, PartialEq)]
struct RoomId {
    room_id: Option<Uuid>,
}

#[component]
pub fn Room() -> impl IntoView {
    let params = use_params::<RoomId>();
    let id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.room_id)
            .unwrap_or_default()
    };
    // todo get room info and have things use that like the title etc 
    // and set page metadata like title
    let room = move || {
        let replying: RwSignal<Option<MessageReply>> = RwSignal::new(None);
        let room_id = id();

        view! {
            <ChatLog replying=replying room=room_id />
            <MessageInput replying=replying room=room_id />
        }
    };

    view! {{room}}
}