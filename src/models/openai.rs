use std::pin::Pin;

use anyhow::anyhow;
use futures::stream::Stream;
use reqwest::Response;
use serde_json::json;
use trpl::{ReceiverStream, channel};

use crate::{completion::Completion, config::OpenAIConfig, prompt::Prompt, response::ChatResponse};

async fn api(config: &OpenAIConfig, prompt: &Prompt, stream: bool) -> anyhow::Result<Response> {
    let OpenAIConfig {
        model,
        base_url,
        api_key,
    } = config;
    let client = reqwest::Client::new();
    let response = client
        .post(base_url)
        .bearer_auth(api_key)
        .json(&json!({
            "model": model,
            "messages": prompt,
            "stream": stream,
            "stream_options": {
                "include_usage": true
            }
        }))
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(anyhow!(
            "request failed with status code {}: {}",
            response.status(),
            response.text().await.unwrap_or_default()
        ));
    }
    Ok(response)
}

pub(crate) async fn stream_openai(
    config: &OpenAIConfig,
    prompt: &Prompt,
) -> anyhow::Result<Pin<Box<dyn Stream<Item = Completion> + Send + Sync>>> {
    let mut response = api(config, prompt, true).await?;
    let (tx, rx) = channel();
    tokio::spawn(async move {
        while let Some(chunk) = response.chunk().await.unwrap() {
            let text = String::from_utf8_lossy(&chunk).to_string();
            for line in text.split("\n\n") {
                if let Some(data) = line.strip_prefix("data: ") {
                    if !data.starts_with("{") {
                        continue;
                    }
                    let response = serde_json::from_str::<ChatResponse>(data).unwrap();
                    tx.send(response.into()).unwrap();
                }
            }
        }
    });
    Ok(Box::pin(ReceiverStream::new(rx)))
}

pub(crate) async fn completion_openai(
    config: &OpenAIConfig,
    prompt: &Prompt,
) -> anyhow::Result<Completion> {
    let response = api(config, prompt, false)
        .await?
        .json::<ChatResponse>()
        .await?;
    Ok(response.into())
}
