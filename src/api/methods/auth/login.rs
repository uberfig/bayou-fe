use gloo_net::http::Request;

use crate::{api::types::{auth_token::DBAuthToken, login_err::LoginErr, login_request::LoginRequest}, state::State};

const PATH: &'static str = "/api/bayou_v1/login";

pub async fn login(state: &State, request: &LoginRequest) -> Result<DBAuthToken, LoginErr> {
    let result = Request::post(&format!("{}{PATH}", state.get_prefix()))
        .header("content-type", "application/json")
        .body(serde_json::to_string(&request).expect("failed to serialize"))
        .unwrap()
        .send()
        .await;
    let result = result.unwrap();
    match result.ok() {
        true => Ok(result.json().await.unwrap()),
        false => Err(result.json().await.unwrap()),
    }
}