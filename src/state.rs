pub struct State {
    // domain: Option<String>,
    prefix: String,
}

impl State {
    pub fn get_prefix(&self) -> &str {
        &self.prefix
    }
}