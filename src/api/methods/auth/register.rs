use gloo_net::http::Request;

use crate::{api::types::devices::{device_info::DeviceInfo, registered_device::RegisteredDevice}, state::State};

const PATH: &'static str = "/api/bayou_v1/register";

pub async fn register_device(state: &State, info: DeviceInfo) -> Result<RegisteredDevice, ()> {
    let result = Request::post(&format!("{}{PATH}", state.get_prefix()))
        .header("content-type", "application/json")
        .body(serde_json::to_string(&info).expect("failed to serialize"))
        .unwrap()
        .send()
        .await;
    let result = result.unwrap();
    let device: RegisteredDevice = result.json().await.unwrap();
    Ok(device)
}