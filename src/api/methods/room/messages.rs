use gloo_net::http::Request;
use leptos::leptos_dom::logging::console_log;
use uuid::Uuid;

use crate::{
    api::types::{api_message::ApiMessage, auth_token::AuthToken},
    state::State,
};

const PATH: &'static str = "/api/bayou_v1/room/messages";

#[derive(Debug, Clone, Copy)]
pub enum MessageSelector {
    /// get the latest messages in a room
    Latest,
    /// get messages older than the provided id
    Older(Uuid),
    /// get messages newer than the provided id
    Newer(Uuid),
}

pub async fn get_messages(
    state: State,
    auth: AuthToken,
    room: Uuid,
    inclusive: bool,
    selector: MessageSelector,
) -> Result<Vec<ApiMessage>, ()> {
    let mut query = format!("?room={}&inclusive={}", room.as_simple(), inclusive);
    match selector {
        MessageSelector::Latest => {},
        MessageSelector::Older(uuid) => query.push_str(&format!("&older={}", uuid.as_simple())),
        MessageSelector::Newer(uuid) => query.push_str(&format!("&newer={}", uuid.as_simple())),
    };
    let link = format!("{}{}{}", state.get_prefix(), PATH, query);

    console_log(&link);

    let result = Request::get(&link)
        .header("content-type", "application/json")
        .header(
            "authorization",
            &serde_json::to_string(&auth).expect("failed to serialize"),
        )
        .send()
        .await;
    let result = result.unwrap();
    match result.ok() {
        true => Ok(result.json().await.unwrap()),
        false => Err(()),
    }
}
