use serde::Deserialize;

use crate::{completion::Completion, usage::Usage};

#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    choices: Vec<Choice>,
    #[serde(default)]
    usage: Option<Usage>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    #[serde(default)]
    message: Option<Content>,
    #[serde(default)]
    delta: Option<Content>,
}

#[derive(Deserialize, Debug)]
struct Content {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    reasoning_content: Option<String>,
}

impl From<ChatResponse> for Completion {
    fn from(response: ChatResponse) -> Self {
        let mut content = None;
        let mut reasoning_content = None;
        if let Some(choice) = response.choices.first() {
            if let Some(message) = &choice.message {
                content = message.content.clone();
                reasoning_content = message.reasoning_content.clone();
            }
            if let Some(delta) = &choice.delta {
                content = delta.content.clone();
                reasoning_content = delta.reasoning_content.clone();
            }
        };
        Completion {
            content,
            reasoning_content,
            usage: response.usage,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_der_response() {
        let json = r#"{"choices":[{"finish_reason":"stop","delta":{"content":"你好"},"index":0,"logprobs":null}],"object":"chat.completion.chunk","usage":null,"created":1762759403,"system_fingerprint":null,"model":"qwen-turbo-latest","id":"chatcmpl-770e00d6-1452-496c-9792-3379d10d6c01"}"#;
        let response = serde_json::from_str::<ChatResponse>(json).unwrap();
        println!("{response:?}");
    }
}
