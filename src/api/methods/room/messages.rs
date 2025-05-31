use gloo_net::http::Request;
use uuid::Uuid;

use crate::{
    api::types::{api_message::ApiMessage, auth_token::AuthToken},
    state::State,
};

const PATH: &'static str = "/api/bayou_v1/todo";

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
    selector: MessageSelector,
) -> Result<Vec<ApiMessage>, ()> {
    let query = match selector {
        MessageSelector::Latest => "",
        MessageSelector::Older(uuid) => &format!("&older={}", uuid.as_simple()),
        MessageSelector::Newer(uuid) => &format!("&newer={}", uuid.as_simple()),
    };
    let link = format!("{}{}{}", state.get_prefix(), PATH, query);
    todo!()
}
