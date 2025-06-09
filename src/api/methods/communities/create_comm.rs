use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

use crate::{
    api::types::{api_community::ApiCommunity, auth_token::AuthToken},
    state::State,
};

const PATH: &'static str = "/api/bayou_v1/community/create";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Communityinfo {
    pub name: String,
    pub description: Option<String>,
}

pub async fn create_comm(
    state: State,
    token: AuthToken,
    comm: Communityinfo,
) -> Result<ApiCommunity, ()> {
    let result = Request::post(&format!("{}{PATH}", state.get_prefix()))
        .header("content-type", "application/json")
        .header(
            "authorization",
            &serde_json::to_string(&token).expect("failed to serialize"),
        )
        .body(serde_json::to_string(&comm).expect("failed to serialize"))
        .expect("invalid body")
        .send()
        .await;
    let result = result.unwrap();
    match result.ok() {
        true => Ok(result.json().await.unwrap()),
        false => Err(()),
    }
}
