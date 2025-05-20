use gloo_net::http::Request;

use crate::state::State;

const PATH: &'static str = "/api/bayou_v1/username_availible";

pub async fn is_username_available(state: &State, uname: &str) -> Result<bool, ()> {
    let result = Request::get(&format!("{}{}/{}", state.get_prefix(), PATH, uname))
        .header("content-type", "application/json")
        .send()
        .await;
    Ok(result.unwrap().ok())
}
