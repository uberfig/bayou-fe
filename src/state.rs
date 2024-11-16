#[derive(Clone)]
pub struct State {
    pub domain: String,
    pub limit: usize,
}

impl State {
    pub fn get_older(&self, post_id: &str) -> String {
        format!(
            "https://{}//api/v1/timelines/public?max_id={}&limit={}",
            &self.domain, post_id, self.limit
        )
    }
    pub fn get_timeline(&self) -> String {
        format!("https://{}/api/v1/timelines/public?limit={}", &self.domain, &self.limit)
    }
}
