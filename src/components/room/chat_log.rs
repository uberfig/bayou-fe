use leptos::{prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_router::{hooks::use_params, params::Params};
use leptos_use::{
    core::ConnectionReadyState, storage::use_local_storage, use_websocket_with_options,
    ReconnectLimit, UseWebSocketOptions, UseWebSocketReturn,
};
use uuid::Uuid;

use crate::{
    api::{
        methods::room::messages::{get_messages, MessageSelector},
        types::{api_message::ApiMessage, auth_token::AuthToken, socket_msg::SocketMsg},
    },
    components::room::segment::{Segment, SegmentList},
    state::{State, AUTH_TOKEN},
};

#[component]
pub fn Loader(
    room: Uuid,
    loading: RwSignal<bool>,
    oldest: RwSignal<Option<Uuid>>,
    log: RwSignal<Vec<Segment>>,
) -> impl IntoView {
    let state = use_context::<ReadSignal<State>>().expect("state should be provided");
    let (logged_in, _, _) = use_local_storage::<Option<AuthToken>, JsonSerdeCodec>(AUTH_TOKEN);

    let load_older = move || {
        loading.set(true);
        let completed = move |messages: &Vec<ApiMessage>| {
            loading.set(false);
            let last = messages.last().map(|x| x.id);
            oldest.set(last);
        };
        log.update(|log| {
            log.insert(
                0,
                load(
                    state.get_untracked(),
                    logged_in.get_untracked().expect("not logged in"),
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
            let posts = get_messages(state, auth, room, selector).await;
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
pub fn ChatLog(room: Uuid) -> impl IntoView {
    let (logged_in, _, _) = use_local_storage::<Option<AuthToken>, JsonSerdeCodec>(AUTH_TOKEN);
    let loading = RwSignal::new(true);
    let state = use_context::<ReadSignal<State>>().expect("state should be provided");
    let oldest: RwSignal<Option<Uuid>> = RwSignal::new(None);
    let completed = move |messages: &Vec<ApiMessage>| {
        loading.set(false);
        let last = messages.last().map(|x| x.id);
        oldest.set(last);
    };
    // log in form oldest segment to newest segment
    let log: RwSignal<Vec<Segment>> = RwSignal::new(vec![
        // load(state.get_untracked(), logged_in.get_untracked().expect("not logged in"), room, MessageSelector::Latest, completed),
        Segment::Live(Vec::new()),
    ]);

    let UseWebSocketReturn {
        ready_state,
        message,
        send,
        // open,
        // close,
        ..
    } = use_websocket_with_options::<AuthToken, SocketMsg, JsonSerdeCodec, _, _>(
        "ws://127.0.0.1:8020/api/bayou_v1/ws",
        UseWebSocketOptions::default()
            .immediate(true)
            .reconnect_limit(ReconnectLimit::Infinite), // .on_open()
    );

    Effect::new(move |_| {
        if ConnectionReadyState::Open == ready_state.get() {
            send(&logged_in.get_untracked().expect("not logged in"));
        }
    });
    // open();
    // send(&logged_in.get_untracked().expect("not logged in"));
    Effect::new(move |_| {
        message.with(move |message| {
            if let Some(message) = message {
                match message {
                    SocketMsg::NewMessage(message) => {
                        log.update(|log| {
                            if let Some(Segment::Live(last)) = log.last_mut() {
                                last.push(message.to_owned());
                            } else {
                                log.push(Segment::Live(vec![message.to_owned()]));
                            }
                        });
                    }
                    SocketMsg::SystemMessage(_) => todo!(),
                }
            }
        })
    });

    view! {<SegmentList segments=log />}
}

#[derive(Params, PartialEq)]
struct RoomId {
    room_id: Option<Uuid>,
}

#[component]
pub fn ChatLogWrap() -> impl IntoView {
    let params = use_params::<RoomId>();
    let id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.room_id)
            .unwrap_or_default()
    };
    let rendered = move || {
        let id = id();
        view! {<ChatLog room=id />}
    };
    view! {{rendered}}
}
