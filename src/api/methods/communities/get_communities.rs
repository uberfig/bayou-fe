use gloo_net::http::Request;

use crate::{
    api::types::{api_community::ApiCommunity, auth_token::AuthToken},
    state::State,
};

const PATH: &'static str = "/api/bayou_v1/community/joined";

/// error if invalid token or failed to connect to server, todo add better error handling for different codes
pub async fn joined_communites(state: State, token: AuthToken) -> Result<Vec<ApiCommunity>, ()> {
    let result = Request::get(&format!("{}{PATH}", state.get_prefix()))
        .header("content-type", "application/json")
        .header("authorization", &serde_json::to_string(&token).expect("failed to serialize"))
        .send()
        .await;
    let result = result.unwrap();
    match result.ok() {
        true => Ok(result.json().await.unwrap()),
        false => Err(()),
    }
}
