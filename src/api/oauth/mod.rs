//! types and links for oauth flow, see https://docs.joinmastodon.org/methods/oauth/
//! 
//! typical app flow as I currently understand it as follows
//! 1. [register application](https://docs.joinmastodon.org/methods/apps/#create) by posting a [`application::RegisterApplication`] to /api/v1/apps
//! 2. get the response [`application::RegisterApplicationResult`] and [verify the app works](https://docs.joinmastodon.org/methods/apps/#verify_credentials)
//! 3. login a user..

pub mod scopes;
pub mod application;
