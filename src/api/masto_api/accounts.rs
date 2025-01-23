use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

use crate::{api::masto_types::account::Account, state::State};

/// the account being looked up may not exist.
///
/// see [`Webfinger::webfinger_request`] for link generation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Webfinger {
    #[serde(flatten)]
    pub result: Option<Account>,
}

impl Webfinger {
    /// oauth: public
    ///
    /// https://docs.joinmastodon.org/methods/accounts/#lookup
    pub fn webfinger_request(state: &State, acct: &str) -> String {
        format!(
            "https://{}/api/v1/accounts/lookup?acct={}",
            &state.domain, acct
        )
    }
}

/// oauth: public
///
/// https://docs.joinmastodon.org/methods/accounts/#lookup
pub async fn webfinger_account(link: String) -> Option<Account> {
    let fetched_acct: Webfinger = Request::get(&link)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    fetched_acct.result
}
