use gloo_net::http::Request;
use uuid::Uuid;

use crate::{
    api::types::{auth_token::AuthToken, room::Room},
    state::State,
};

const PATH: &'static str = "/api/bayou_v1/community/rooms";

/// error if invalid token or failed to connect to server, todo add better error handling for different codes
pub async fn community_rooms(
    state: State,
    token: AuthToken,
    community: Uuid,
) -> Result<Vec<Room>, ()> {
    let result = Request::get(&format!("{}{}/{}", state.get_prefix(), PATH, community))
        .header("content-type", "application/json")
        .header(
            "authorization",
            &serde_json::to_string(&token).expect("failed to serialize"),
        )
        .send()
        .await;
    let result = result.unwrap();
    match result.ok() {
        true => Ok(result.json().await.unwrap()),
        false => Err(()),
    }
}
