use regex::Regex;
use serde::{Deserialize, Serialize};
use url::Url;

/// https://docs-p.joinmastodon.org/entities/CustomEmoji/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomEmoji {
    /// The name of the custom emoji.
    pub shortcode: String,
    /// A link to the custom emoji.
    pub url: Url,
    /// A link to a static copy of the custom emoji.
    pub static_url: Url,
    /// Whether this Emoji should be visible in the picker or unlisted.
    pub visible_in_picker: Option<bool>,
    /// Used for sorting custom emoji in the picker.
    pub category: Option<String>,
}

impl CustomEmoji {
    pub fn parse_emoji(emojis: &Vec<CustomEmoji>, content: &str) -> String {
        let mut content = content.to_string();
        for emoji in emojis {
            if let Ok(regex) = Regex::new(&format!(":{}:", &emoji.shortcode)) {
                let replacement = &format!(r#"<img src="{}" alt="{} />""#, emoji.url.as_str(), emoji.shortcode);
                content = regex.replace_all(&content, replacement).to_string();
            }
            
        }
        return content;
    }
}