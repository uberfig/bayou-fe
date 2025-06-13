use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]

pub enum RequestErr {
    Unauthorized,
    Unkown,
}