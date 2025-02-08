use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

use super::scopes::Scopes;
// use super::serde_fns::*;

/// use by posting to /api/v1/apps with Content-Type: application/json
///
/// https://docs.joinmastodon.org/methods/apps/#create-request-example
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterApplication {
    pub client_name: String,
    /// Where the user should be redirected after authorization.
    /// To display the authorization code to the user instead of redirecting to a web page,
    /// use urn:ietf:wg:oauth:2.0:oob in this parameter.
    pub redirect_uris: Vec<String>,
    /// Space separated list of scopes. If none is provided, defaults to read.
    /// See [OAuth Scopes](https://docs.joinmastodon.org/api/oauth-scopes/) for a list of possible scopes.
    pub scopes: Scopes,
    /// URL to the homepage of your app
    pub website: String,
}
impl RegisterApplication {
    pub async fn post(self, domain: String) -> Result<RegisterApplicationResult, ()> {
        let result = Request::post(&format!("https://{}//api/v1/apps", domain))
            .send()
            .await;
        match result {
            Ok(result) => {
                let result: Result<RegisterApplicationResult, _> = result.json().await;
                match result {
                    Ok(result) => Ok(result),
                    Err(err) => todo!()
                }
            },
            Err(err) => todo!(),
        }
    }
}
// async fn fetch_posts(segment_link: String, set_oldest: WriteSignal<FeedPos>) -> Vec<Status> {
//     log!("segment_link: {}", &segment_link);
//     let fetched_posts: Vec<Status> = Request::get(&segment_link)
//         .send()
//         .await
//         .expect("invalid response")
//         .json()
//         .await
//         .expect("deserialization error");
//     set_oldest.update(|x| {
//         match fetched_posts.last() {
//             Some(post) => x.oldest_id = Some(post.id.clone()),
//             None => x.end_of_feed = true,
//         };
//     });
//     fetched_posts
// }

/// result for registering an application.
/// from mastodon: Treat the [`RegisterApplicationResult::client_id`]
/// and [`RegisterApplicationResult::client_secret`] properties as if
/// they are passwords. We recommend you encrypt these when storing
/// in your cache, to prevent credential exposure.
///
/// https://docs.joinmastodon.org/methods/apps/#create-request-example
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterApplicationResult {
    pub id: String,
    pub name: String,
    /// URL to the homepage of your app
    pub website: String,
    /// See [OAuth Scopes](https://docs.joinmastodon.org/api/oauth-scopes/) for a list of possible scopes.
    pub scopes: Vec<String>,
    // The redirect_uri property is considered deprecated as of 4.3.0 and
    // should not be used, instead use the redirect_uris property.
    // pub redirect_uri: String,
    /// Where the user should be redirected after authorization.
    /// To display the authorization code to the user instead of redirecting to a web page,
    /// use urn:ietf:wg:oauth:2.0:oob in this parameter.
    pub redirect_uris: Vec<String>,
    pub client_id: String,
    pub client_secret: String,
    /// not totally sure how this works atm
    pub client_secret_expires_at: Option<usize>,
    /// I think this is depreciated, don't use unless necessary
    pub vapid_key: Option<String>,
}
