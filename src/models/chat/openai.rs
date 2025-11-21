use std::fmt::Display;

use anyhow::anyhow;
use async_stream::stream;
use futures::StreamExt;
use serde::Deserialize;
use serde_json::json;

use crate::{Completion, OpenAIModelOptions, Prompt, Stream, Usage};

async fn api(
    prompt: &Prompt,
    options: &OpenAIModelOptions,
    stream: bool,
) -> anyhow::Result<reqwest::Response> {
    let OpenAIModelOptions {
        model,
        base_url,
        api_key,
    } = options;
    if base_url.is_none() {
        return Err(anyhow!("'base_url' is required"));
    }
    let client = reqwest::Client::new();
    let mut request = client.post(base_url.as_ref().unwrap()).json(&json!({
        "model": model,
        "messages": prompt,
        "stream": stream,
        "stream_options": {
            "include_usage": true
        }
    }));
    if let Some(api_key) = api_key {
        request = request.bearer_auth(api_key);
    }
    let response = request.send().await?;
    if !response.status().is_success() {
        return Err(anyhow!(
            "request failed with status code {}: {}",
            response.status(),
            response.text().await.unwrap_or_default()
        ));
    }
    Ok(response)
}

#[derive(Deserialize, Debug)]
pub(crate) struct Response {
    pub choices: Vec<Choice>,
    #[serde(default)]
    pub usage: Option<Usage>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Choice {
    #[serde(default)]
    pub message: Option<Content>,
    #[serde(default)]
    pub delta: Option<Content>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Content {
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub reasoning_content: Option<String>,
}

impl Response {
    pub fn content(&self) -> Option<&String> {
        if let Some(Choice { message, delta }) = self.choices.first() {
            if let Some(Content { content, .. }) = message {
                return content.as_ref();
            } else if let Some(Content { content, .. }) = delta {
                return content.as_ref();
            }
        }
        None
    }

    pub fn reasoning_content(&self) -> Option<&String> {
        if let Some(Choice { message, delta }) = self.choices.first() {
            if let Some(Content {
                reasoning_content, ..
            }) = message
            {
                return reasoning_content.as_ref();
            } else if let Some(Content {
                reasoning_content, ..
            }) = delta
            {
                return reasoning_content.as_ref();
            }
        }
        None
    }

    pub fn message(&self) -> Option<&Content> {
        self.choices
            .first()
            .and_then(|choice| choice.message.as_ref())
    }

    pub fn delta(&self) -> Option<&Content> {
        self.choices
            .first()
            .and_then(|choice| choice.delta.as_ref())
    }

    pub fn into_delta(self) -> Option<Content> {
        self.choices
            .into_iter()
            .next()
            .and_then(|choice| choice.delta)
    }
}

pub(crate) async fn completion(
    prompt: &Prompt,
    options: &OpenAIModelOptions,
) -> anyhow::Result<Response> {
    let response = api(prompt, options, false).await?.json().await?;
    Ok(response)
}

impl From<Response> for Completion {
    fn from(response: Response) -> Self {
        Completion {
            content: response.content().cloned(),
            reasoning_content: response.reasoning_content().cloned(),
            usage: response.usage,
        }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        if let Some(Content {
            content: Some(content),
            reasoning_content,
        }) = self.message()
        {
            if let Some(reasoning_content) = reasoning_content {
                result.push_str("<think>");
                result.push_str(reasoning_content);
                result.push_str("</think>");
            }
            result.push_str(content);
        }
        write!(f, "{}", result)
    }
}

pub(crate) async fn stream(
    prompt: &Prompt,
    options: &OpenAIModelOptions,
) -> anyhow::Result<Stream<Response>> {
    let mut response = api(prompt, options, true).await?;
    let stream = stream! {
        'outer: while let Ok(Some(chunk)) = response.chunk().await {
            let text = String::from_utf8_lossy(&chunk).to_string();
            for line in text.split("\n\n") {
                if let Some(data) = line.strip_prefix("data: ") {
                    if let Ok(response) = serde_json::from_str(data) {
                        yield response;
                    } else {
                        break 'outer;
                    }
                }
            }
        }
    };
    Ok(Stream::new(Box::pin(stream)))
}

impl From<Stream<Response>> for Stream<Completion> {
    fn from(stream: Stream<Response>) -> Self {
        stream.into_inner().map(|response| response.into()).into()
    }
}

impl From<Stream<Response>> for Stream<String> {
    fn from(mut stream: Stream<Response>) -> Self {
        let text_stream = stream! {
            let mut reasoning = false;
            while let Some(item) = stream.next().await {
                if let Some(Content {
                    content,
                    reasoning_content,
                }) = item.into_delta()
                {
                    if let Some(reasoning_content) = reasoning_content {
                        if !reasoning {
                            yield "<think>".to_string();
                            reasoning = true;
                        }
                        yield reasoning_content;
                    }
                    if let Some(content) = content {
                        if reasoning {
                            yield "</think>".to_string();
                            reasoning = false;
                        }
                        yield content;
                    }
                } else {
                    break;
                }
            }
        };
        text_stream.into()
    }
}

impl Stream<Response> {
    pub async fn collect(mut self) -> Completion {
        let mut content_completed = None;
        let mut reasoning_content_completed = None;
        let mut usage_completed = None;
        while let Some(item) = self.next().await {
            if let Some(Content {
                content,
                reasoning_content,
            }) = item.delta()
            {
                if let Some(content) = content {
                    *content_completed.get_or_insert_default() += content.as_str();
                }
                if let Some(reasoning_content) = reasoning_content {
                    *reasoning_content_completed.get_or_insert_default() +=
                        reasoning_content.as_str();
                }
            } else if item.usage.is_none() {
                break;
            }
            if let Some(usage) = item.usage {
                usage_completed = Some(usage);
            }
        }
        Completion::new(
            content_completed,
            reasoning_content_completed,
            usage_completed,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_der_response() {
        let json = r#"{"choices":[{"finish_reason":"stop","delta":{"content":"你好"},"index":0,"logprobs":null}],"object":"chat.completion.chunk","usage":null,"created":1762759403,"system_fingerprint":null,"model":"qwen-turbo-latest","id":"chatcmpl-770e00d6-1452-496c-9792-3379d10d6c01"}"#;
        let response = serde_json::from_str::<Response>(json).unwrap();
        println!("{response:?}");
    }
}
