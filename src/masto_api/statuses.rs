use gloo_net::http::Request;
use crate::{masto_types::status::Status, state::State};

///  Public for public statuses, user token + read:statuses for private statuses
///
/// https://docs.joinmastodon.org/methods/statuses/#get
pub fn status_request_link(state: &State, id: &str) -> String {
    format!(
        "https://{}/api/v1/statuses/{}",
        &state.domain, id
    )
}

/// oauth: public
///
/// https://docs.joinmastodon.org/methods/accounts/#lookup
pub async fn request_status(link: String) -> Option<Status> {
    Request::get(&link)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .ok()
}
