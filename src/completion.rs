use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::usage::Usage;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Completion {
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub reasoning_content: Option<String>,
    #[serde(default)]
    pub usage: Option<Usage>,
}

impl Completion {
    pub(crate) fn new(
        content: Option<String>,
        reasoning_content: Option<String>,
        usage: Option<Usage>,
    ) -> Self {
        Self {
            content,
            reasoning_content,
            usage,
        }
    }
}

impl Display for Completion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();
        if let Some(reasoning_content) = &self.reasoning_content {
            text.push_str("<think>");
            text.push_str(reasoning_content);
            text.push_str("</think>");
        }
        if let Some(content) = &self.content {
            text.push_str(content);
        }
        write!(f, "{}", text)
    }
}
