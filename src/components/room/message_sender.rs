use leptos::{prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_use::storage::use_local_storage;
use uuid::Uuid;

use crate::{
    api::{methods::message::send_message::send_message, 
        types::{
            auth_token::AuthToken, message_info::Messageinfo, text_format::TextFormat,
        }}
    ,
    state::{State, AUTH_TOKEN},
};

#[derive(Debug, Clone)]
pub struct MessageReply {
    pub display_name: String,
    pub message_id: Uuid,
}

pub fn send<F>(
    state: State,
    auth: AuthToken,
    message: Messageinfo,
    completed: F,
) -> LocalResource<Result<(), ()>>
where
    F: FnOnce(bool) + Clone + 'static,
{
    let loader = LocalResource::new(move || {
        let state = state.to_owned();
        let auth = auth.to_owned();
        let message = message.to_owned();
        let completed = completed.clone();
        async move {
            let result = send_message(state, auth, message).await;
            completed(result.is_ok());
            result
        }
    });

    loader
}

#[component]
pub fn MessageInput(replying: RwSignal<Option<MessageReply>>, room: Uuid) -> impl IntoView {
    let message = RwSignal::new("".to_string());

    let loading = RwSignal::new(false);
    let send_result: RwSignal<Option<LocalResource<Result<(), ()>>>> =
        RwSignal::new(None);

    let (auth, _, _) =
        use_local_storage::<Option<AuthToken>, JsonSerdeCodec>(AUTH_TOKEN);
    let state = use_context::<ReadSignal<State>>().expect("state should be provided");

    let replying_disp = move || {
        match replying.get() {
            Some(reply_to) => view! {
                <p>"replying to: "{reply_to.display_name}</p>
            }.into_any(),
            None => view! {}.into_any(),
        }
    };

    let finished_sending = move |success: bool| {
        if success {
            message.set("".to_string());
        }
        loading.set(false);
    };

    let send_clicked = move || {
        loading.set(true);
        let message = Messageinfo {
            is_reply: replying.get_untracked().is_some(), 
            in_reply_to: replying.get_untracked().map(|x| x.message_id), 
            proxy_id: None, 
            content: message.get_untracked(), 
            format: TextFormat::Markdown, 
            language: None, 
            room
        };
        send_result.set(Some(send(state.get_untracked(), auth.get_untracked().expect("not logged in"), message, finished_sending)));
    };

    view! {
        {replying_disp}
        <form>
            <p>
                <input type="text" id="text"
                        bind:value=message
                        disabled=move || loading.get()
                        required
                />
                <button type="submit"
                    on:click=move |ev| {
                        ev.prevent_default();
                        send_clicked();
                    }
                    disabled=move || loading.get()
                >
                "Send"
                </button>
            </p>
        </form>
        <Show when=move || loading.get()>
            <p>"sending..."</p>
        </Show>
    }
}
