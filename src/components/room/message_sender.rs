use leptos::{html::Input, prelude::*};
use uuid::Uuid;

use crate::{
    api::{
        methods::message::send_message::send_message,
        types::{auth_token::AuthToken, message_info::Messageinfo, text_format::TextFormat},
    },
    state::State,
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
    let send_result: RwSignal<Option<LocalResource<Result<(), ()>>>> = RwSignal::new(None);

    let auth = use_context::<Signal<Option<AuthToken>>>().expect("token should be provided");
    let state = use_context::<ReadSignal<State>>().expect("state should be provided");

    let replying_disp = move || match replying.get() {
        Some(reply_to) => view! {
            <p>
                "replying to: "{reply_to.display_name}
                <button
                    on:click=move |_| {
                        replying.set(None);
                    }
                >
                "cancel"
            </button>
            </p>
        }
        .into_any(),
        None => view! {}.into_any(),
    };

    let node_ref = NodeRef::<Input>::new();

    let finished_sending = move |success: bool| {
        if success {
            message.set("".to_string());
        }
        loading.set(false);
        if let Some(input) = node_ref.get_untracked() {
            input.set_disabled(false);
            let _ = input.focus();
        }
        replying.set(None);
    };

    let send_clicked = move || {
        loading.set(true);
        if let Some(input) = node_ref.get_untracked() {
            input.set_disabled(true);
        }
        let message = Messageinfo {
            is_reply: replying.get_untracked().is_some(),
            in_reply_to: replying.get_untracked().map(|x| x.message_id),
            proxy_id: None,
            content: message.get_untracked(),
            format: TextFormat::Markdown,
            language: None,
            room,
        };
        send_result.set(Some(send(
            state.get_untracked(),
            auth.get_untracked().unwrap_or_default(),
            message,
            finished_sending,
        )));
    };

    view! {
        {replying_disp}
        <form>
            <p>
                <input type="text" id="text" node_ref=node_ref
                        autofocus=true
                        bind:value=message
                        // disabled=move || loading.get()
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
