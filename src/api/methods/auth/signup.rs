use gloo_net::http::Request;

use crate::{
    api::types::{signup_result::SignupResult, signup_user::SignupUser},
    state::State,
};

const PATH: &'static str = "/api/bayou_v1/signup";

pub async fn signup(state: &State, user: &SignupUser) -> Result<(), SignupResult> {
    let result = Request::post(&format!("{}{PATH}", state.get_prefix()))
        .header("content-type", "application/json")
        .body(serde_json::to_string(&user).expect("failed to serialize"))
        .unwrap()
        .send()
        .await;
    let result = result.unwrap();
    match result.ok() {
        true => Ok(()),
        false => Err(result.json().await.unwrap_or(SignupResult::UnkownFailure)),
    }
}
