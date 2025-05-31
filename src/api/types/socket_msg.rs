use serde::{Deserialize, Serialize};

use super::api_message::ApiMessage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SocketMsg {
    NewMessage(ApiMessage),
    SystemMessage(String),
}
