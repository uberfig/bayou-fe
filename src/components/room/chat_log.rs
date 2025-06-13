use leptos::{leptos_dom::logging::console_log, prelude::*};
use uuid::Uuid;

use crate::{
    api::{
        methods::room::messages::{get_messages, MessageSelector},
        types::{api_message::ApiMessage, auth_token::AuthToken, socket_msg::SocketMsg},
    },
    components::room::segment::{Segment, SegmentList},
    state::State,
};

use super::message_sender::MessageReply;

#[component]
pub fn Loader(
    room: Uuid,
    loading: RwSignal<bool>,
    oldest: RwSignal<Option<Uuid>>,
    log: RwSignal<Vec<Segment>>,
) -> impl IntoView {
    let state = use_context::<ReadSignal<State>>().expect("state should be provided");
    let logged_in = use_context::<Signal<Option<AuthToken>>>().expect("token should be provided");

    let load_older = move || {
        loading.set(true);
        let completed = move |messages: &Vec<ApiMessage>| {
            let last = messages.get(0).map(|x| x.id);
            oldest.set(last);
            loading.set(false);
        };
        log.update(|log| {
            log.insert(
                0,
                load(
                    state.get_untracked(),
                    logged_in.get_untracked().unwrap_or_default(),
                    room,
                    MessageSelector::Older(
                        oldest
                            .get_untracked()
                            .expect("can't get older, end of feed"),
                    ),
                    completed,
                ),
            );
        });
    };
    let display = move || match loading.get() {
        true => view! {<p>"loading..."</p>}.into_any(),
        false => view! {
            <button
                on:click=move |_| {load_older();}
            >
            "Load older"
            </button>
        }
        .into_any(),
    };
    let end_of_messages = move || match oldest.get() {
        Some(_) => display().into_any(),
        None => view! {<p>"beginning of room"</p>}.into_any(),
    };
    view! {
        {end_of_messages}
    }
}

pub fn load<F>(
    state: State,
    auth: AuthToken,
    room: Uuid,
    selector: MessageSelector,
    completed: F,
) -> Segment
where
    F: FnOnce(&Vec<ApiMessage>) + Clone + 'static,
{
    let loader = LocalResource::new(move || {
        let state = state.to_owned();
        let auth = auth.to_owned();
        let completed = completed.clone();
        async move {
            let posts = get_messages(state, auth, room, false, selector).await;
            match posts {
                Ok(posts) => {
                    completed(&posts);
                    posts
                }
                Err(_err) => todo!(),
            }
        }
    });

    Segment::Loaded(loader)
}

#[component]
pub fn ChatLog(replying: RwSignal<Option<MessageReply>>, room: Uuid, message: Signal<Option<SocketMsg>>) -> impl IntoView {
    let logged_in = use_context::<Signal<Option<AuthToken>>>().expect("token should be provided");
    let loading = RwSignal::new(true);
    let state = use_context::<ReadSignal<State>>().expect("state should be provided");
    let oldest: RwSignal<Option<Uuid>> = RwSignal::new(None);
    let completed = move |messages: &Vec<ApiMessage>| {
        loading.set(false);
        let last = messages.get(0).map(|x| x.id);
        oldest.set(last);
    };
    // log in form oldest segment to newest segment
    let log: RwSignal<Vec<Segment>> = RwSignal::new(vec![
        load(
            state.get_untracked(),
            logged_in.get_untracked().unwrap_or_default(),
            room,
            MessageSelector::Latest,
            completed,
        ),
        Segment::Live(Vec::new()),
    ]);

    Effect::new(move |_| {
        console_log(
            &oldest
                .get()
                .map(|x| x.as_hyphenated().to_string())
                .unwrap_or("no oldest".to_string()),
        );
    });

    Effect::new(move |_| {
        let log: Vec<String> = log
            .get()
            .into_iter()
            .map(|x| match x {
                Segment::Loaded(_) => "loaded".to_string(),
                Segment::Live(_) => "live messages".to_string(),
            })
            .collect();
        console_log(&serde_json::to_string(&log).unwrap());
    });

    Effect::new(move |_| {
        console_log("message recieved");
        message.with(move |message| {
            if let Some(message) = message {
                match message {
                    SocketMsg::NewMessage(message) => {
                        match message.room == room {
                            true => {
                                // we may also want to include a check for if the window is focused
                                log.update(|log| {
                                    if let Some(Segment::Live(last)) = log.last_mut() {
                                        last.push(message.to_owned());
                                    } else {
                                        log.push(Segment::Live(vec![message.to_owned()]));
                                    }
                                });
                            }
                            // we will want to send a notification to the user
                            false => {
                                console_log("message in other room, todo impliment notifications");
                                console_log(&serde_json::to_string_pretty(&message).unwrap());
                            }, 
                        }
                    }
                    SocketMsg::SystemMessage(_) => todo!(),
                }
            }
        })
    });

    view! {
        <Loader log=log room=room oldest=oldest loading=loading/>
        <SegmentList segments=log replying=replying />
    }
}
