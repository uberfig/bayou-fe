use gloo_net::http::Request;

use crate::{
    api::types::{auth_token::AuthToken, message_info::Messageinfo},
    state::State,
};

const PATH: &'static str = "/api/bayou_v1/message/new";

pub async fn send_message(state: State, token: AuthToken, message: Messageinfo) -> Result<(), ()> {
    let result = Request::post(&format!("{}{PATH}", state.get_prefix()))
        .header("content-type", "application/json")
        .header(
            "authorization",
            &serde_json::to_string(&token).expect("failed to serialize"),
        )
        .body(serde_json::to_string(&message).expect("failed to serialize"))
        .expect("invalid body")
        .send()
        .await;
    let result = result.unwrap();
    match result.ok() {
        true => Ok(()),
        false => Err(()),
    }
}
