use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiCommunity {
    pub id: Uuid,
    pub domain: String,
    pub name: String,
    pub description: Option<String>,
    pub owner: Uuid,
    pub created: i64,
}

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

impl ApiCommunity {
    pub fn get_abbrv(&self) -> String {
        let mut iter = self.name.trim().split_ascii_whitespace();
        let first = iter.next().unwrap_or("err");
        if let Some(second) = iter.next() {
            if let Some(third) = iter.next() {
                return format!("{}{}{}", first.chars().next().unwrap(), second.chars().next().unwrap(), third.chars().next().unwrap());
            }
            return format!("{}{}", first.chars().next().unwrap(), second.chars().next().unwrap());
        }
        truncate(first, 3).to_string()
    }
}