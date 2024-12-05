use crate::state::State;

/// OAuth: Public. Requires app token + read:statuses if the instance has disabled public preview.
/// 
/// use max_id to get posts older than a post, use min_id to get newer. 
/// returns an array of [`crate::masto_types::status::Status`]
/// 
/// https://docs.joinmastodon.org/methods/timelines/#public
pub fn public_timeline(state: &State, max_id: Option<&str>, min_id: Option<&str>) -> String {
    let mut link = format!(
        "https://{}/api/v1/timelines/public?limit={}",
        &state.domain, &state.limit
    );
    if let Some(max_id) = max_id {
        link = format!("{}&max_id={}", link, max_id);
    }
    if let Some(min_id) = min_id {
        link = format!("{}&min_id={}", link, min_id);
    }
    link
}
