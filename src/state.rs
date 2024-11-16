#[derive(Clone)]
pub struct State {
    pub domain: String,
    pub limit: usize,
}

impl State {
    pub fn get_older_link(&self, post_id: &str, feed: Feed) -> String {
        match feed {
            Feed::Public => {
                format!(
                    "https://{}//api/v1/timelines/public?max_id={}&limit={}",
                    &self.domain, post_id, self.limit
                )
            },
            Feed::Home => todo!(),
        }
    }
    pub fn get_timeline_link(&self, feed: Feed) -> String {
        match feed {
            Feed::Public => format!("https://{}/api/v1/timelines/public?limit={}", &self.domain, &self.limit),
            Feed::Home => todo!(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Feed {
    Public,
    Home,
}