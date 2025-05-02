use gloo_net::http::Request;

use crate::state::State;

const PATH: &'static str = "/api/bayou_v1/username_availible";

pub async fn is_username_available(state: &State, uname: &str) -> Result<bool, ()> {
    let result = Request::get(&format!("{}{PATH}", state.get_prefix()))
        .header("content-type", "application/json")
        .body(serde_json::to_string(&uname).expect("failed to serialize"))
        .unwrap()
        .send()
        .await;
    Ok(result.unwrap().ok())
}