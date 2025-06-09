use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    api::types::{auth_token::AuthToken, room::{Room, RoomInfo}},
    state::State,
};

const PATH: &'static str = "/api/bayou_v1/community/create_room";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewRoom {
    pub room_info: RoomInfo,
    pub community: Uuid,
}

pub async fn create_room(
    state: State,
    token: AuthToken,
    room: NewRoom,
) -> Result<Room, ()> {
    let result = Request::post(&format!("{}{PATH}", state.get_prefix()))
        .header("content-type", "application/json")
        .header(
            "authorization",
            &serde_json::to_string(&token).expect("failed to serialize"),
        )
        .body(serde_json::to_string(&room).expect("failed to serialize"))
        .expect("invalid body")
        .send()
        .await;
    let result = result.unwrap();
    match result.ok() {
        true => Ok(result.json().await.unwrap()),
        false => Err(()),
    }
}
