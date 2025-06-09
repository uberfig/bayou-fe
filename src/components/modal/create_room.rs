use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos_router::components::Redirect;
use leptos_use::storage::use_local_storage;
use uuid::Uuid;

use crate::{
    api::{
        methods::communities::create_room::{create_room, NewRoom},
        types::{auth_token::AuthToken, room::{Room, RoomInfo}},
    },
    state::{State, AUTH_TOKEN},
};

pub fn create<F>(
    state: State,
    auth: AuthToken,
    room: NewRoom,
    completed: F,
) -> LocalResource<Result<Room, ()>>
where
    F: FnOnce(bool) + Clone + 'static,
{
    let loader = LocalResource::new(move || {
        let state = state.to_owned();
        let auth = auth.to_owned();
        let room = room.to_owned();
        let completed = completed.clone();
        async move {
            let result = create_room(state, auth, room).await;
            completed(result.is_ok());
            result
        }
    });

    loader
}

#[component]
pub fn CreateRoom(display: RwSignal<bool>, refresh: RwSignal<()>, comm: Uuid, room_count: RwSignal<usize>) -> impl IntoView {
    let name = RwSignal::new("".to_string());
    let loading = RwSignal::new(false);
    let empty_display = RwSignal::new(false);
    let create_result: RwSignal<Option<LocalResource<Result<Room, ()>>>> =
        RwSignal::new(None);

    let (auth, _, _) = use_local_storage::<Option<AuthToken>, JsonSerdeCodec>(AUTH_TOKEN);
    let state = use_context::<ReadSignal<State>>().expect("state should be provided");

    let completed = move |_| {
        loading.set(false);
        refresh.set(());
    };

    let create_clicked = move || {
        let name_cleaned = name.get_untracked().trim().to_string();
        name.set(name_cleaned.clone());
        if name_cleaned.is_empty() {
            empty_display.set(true);
            return;
        }
        loading.set(true);

        let room_info = RoomInfo {
            name: name_cleaned, 
            description: None, 
            category: None, 
            display_order: room_count.get_untracked() as i64,
        };
        create_result.set(Some(create(
            state.get_untracked(),
            auth.get_untracked().expect("not logged in"),
            NewRoom { room_info, community: comm },
            completed,
        )));
    };

    let hide = move || {
        create_result.set(None);
        name.set("".to_string());
        empty_display.set(false);
        display.set(false);
    };

    view! {
        <form class="create_room" autofocus>
        <p>
            <label for="name">"room name:"</label>
        </p>
        <p>
            <input type="text" id="name"
                bind:value=name
                required
                disabled=move || loading.get()
            />
            <button type="submit"
                on:click=move |ev| {
                    ev.prevent_default();
                    create_clicked();
                }
                disabled=move || loading.get()
            >
            "Create"
            </button>
            </p>
        </form>
        <button
            on:click=move |_| {
                hide();
            }
        >
        "cancel"
        </button>
        <Show when=move || {
            if let Some(val) = create_result.get() {
                if let Some(val) = val.get() {
                    return val.is_ok();
                }
            }
            false
        }>
            {move || {
                if let Some(val) = create_result.get() {
                    if let Some(val) = val.get() {
                        if let Ok(completed) = val {
                            hide();
                            return view! {
                                <Redirect path=format!("/app/{}/{}", comm.as_simple().to_string(), completed.id.as_simple().to_string())/>
                            }.into_any();
                        }
                    }
                }
                view! {<p>"an unexpected error has occured".to_string()</p>}.into_any()
            }}
        </Show>
        <Show when=move || empty_display.get() >
            <p>"name cannot be empty"</p>
        </Show>
    }
}
