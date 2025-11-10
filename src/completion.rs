use serde::Deserialize;

use crate::usage::Usage;

#[derive(Deserialize, Clone, Debug)]
pub struct Completion {
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub reasoning_content: Option<String>,
    #[serde(default)]
    pub usage: Option<Usage>,
}

impl Completion {
    pub fn with_content(content: String) -> Self {
        Self {
            content: Some(content),
            reasoning_content: None,
            usage: None,
        }
    }

    pub fn with_reasoning_content(reasoning_content: String) -> Self {
        Self {
            content: None,
            reasoning_content: Some(reasoning_content),
            usage: None,
        }
    }

    pub fn with_usage(usage: Usage) -> Self {
        Self {
            content: None,
            reasoning_content: None,
            usage: Some(usage),
        }
    }
}
