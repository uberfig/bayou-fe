use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SignupResult {
    UsernameTaken,
    InvalidToken,
    Success,
    InvalidUsername,
    /// not something the server returns, just used
    /// for if something we don't know how to handle
    /// is returned from the server
    UnkownFailure,
}

impl std::fmt::Display for SignupResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignupResult::UsernameTaken => write!(f, "username is taken"),
            SignupResult::InvalidToken => write!(f, "signup token is invalid"),
            SignupResult::Success => write!(f, "signup successful"),
            SignupResult::InvalidUsername => write!(f, "username contains invalid characters"),
            SignupResult::UnkownFailure => write!(f, "unkown failure"),
        }
    }
}