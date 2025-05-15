use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LoginErr {
    InvalidUsernameOrPassword,
    InvalidDevice,
}

impl std::fmt::Display for LoginErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginErr::InvalidUsernameOrPassword => write!(f, "Invalid Username or Password"),
            LoginErr::InvalidDevice => {
                write!(f, "device not valid, please Refresh the page and try again")
            }
        }
    }
}
