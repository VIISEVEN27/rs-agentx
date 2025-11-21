use anyhow::Ok;
use async_trait::async_trait;
use futures::StreamExt;

use crate::{Completion, Model, ModelOptions, Prompt, Stream};

mod openai;

async fn completion_with(prompt: &Prompt, options: &ModelOptions) -> anyhow::Result<Completion> {
    match options {
        ModelOptions::OpenAI(options) => Ok(openai::completion(prompt, options).await?.into()),
    }
}

async fn text_completion_with(prompt: &Prompt, options: &ModelOptions) -> anyhow::Result<String> {
    match options {
        ModelOptions::OpenAI(options) => Ok(openai::completion(prompt, options).await?.to_string()),
    }
}

#[async_trait]
pub trait ChatModel: Model {
    async fn completion_with(
        &self,
        prompt: &Prompt,
        options: ModelOptions,
    ) -> anyhow::Result<Completion> {
        let options = self.options().clone().merge(options);
        completion_with(prompt, &options).await
    }

    async fn completion(&self, prompt: &Prompt) -> anyhow::Result<Completion> {
        completion_with(prompt, self.options()).await
    }

    async fn text_completion_with(
        &self,
        prompt: &Prompt,
        options: ModelOptions,
    ) -> anyhow::Result<String> {
        let options = self.options().clone().merge(options);
        text_completion_with(prompt, &options).await
    }

    async fn text_completion(&self, prompt: &Prompt) -> anyhow::Result<String> {
        text_completion_with(prompt, self.options()).await
    }
}

async fn stream_with(
    prompt: &Prompt,
    options: &ModelOptions,
) -> anyhow::Result<Stream<Completion>> {
    match options {
        ModelOptions::OpenAI(options) => Ok(openai::stream(prompt, options).await?.into()),
    }
}

async fn text_stream_with(
    prompt: &Prompt,
    options: &ModelOptions,
) -> anyhow::Result<Stream<String>> {
    match options {
        ModelOptions::OpenAI(options) => Ok(openai::stream(prompt, options).await?.into()),
    }
}

async fn completion_from_stream_with(
    prompt: &Prompt,
    options: &ModelOptions,
) -> anyhow::Result<Completion> {
    match options {
        ModelOptions::OpenAI(options) => Ok(openai::stream(prompt, options).await?.collect().await),
    }
}

async fn text_completion_from_stream_with(
    prompt: &Prompt,
    options: &ModelOptions,
) -> anyhow::Result<String> {
    Ok(text_stream_with(prompt, options)
        .await?
        .into_inner()
        .collect()
        .await)
}

#[async_trait]
pub trait StreamingChatModel: Model {
    async fn stream_with(
        &self,
        prompt: &Prompt,
        options: ModelOptions,
    ) -> anyhow::Result<Stream<Completion>> {
        let options = self.options().clone().merge(options);
        stream_with(prompt, &options).await
    }

    async fn stream(&self, prompt: &Prompt) -> anyhow::Result<Stream<Completion>> {
        stream_with(prompt, self.options()).await
    }

    async fn text_stream_with(
        &self,
        prompt: &Prompt,
        options: ModelOptions,
    ) -> anyhow::Result<Stream<String>> {
        let options = self.options().clone().merge(options);
        text_stream_with(prompt, &options).await
    }

    async fn text_stream(&self, prompt: &Prompt) -> anyhow::Result<Stream<String>> {
        text_stream_with(prompt, self.options()).await
    }

    async fn completion_with(
        &self,
        prompt: &Prompt,
        options: ModelOptions,
    ) -> anyhow::Result<Completion> {
        let options = self.options().clone().merge(options);
        completion_from_stream_with(prompt, &options).await
    }

    async fn completion(&self, prompt: &Prompt) -> anyhow::Result<Completion> {
        completion_from_stream_with(prompt, self.options()).await
    }

    async fn text_completion_with(
        &self,
        prompt: &Prompt,
        options: ModelOptions,
    ) -> anyhow::Result<String> {
        let options = self.options().clone().merge(options);
        text_completion_from_stream_with(prompt, &options).await
    }

    async fn text_completion(&self, prompt: &Prompt) -> anyhow::Result<String> {
        text_completion_from_stream_with(prompt, self.options()).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        message::{Message, Role},
        options::{ModelOptions, OpenAIModelOptions},
        prompt::Prompt,
    };

    use super::*;

    struct Qwen {
        options: ModelOptions,
    }

    impl Qwen {
        fn new() -> Self {
            Self {
                options: OpenAIModelOptions::new()
                    .model("qwen-vl-plus-2025-08-15")
                    .base_url("https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions")
                    .api_key("")
                    .into(),
            }
        }
    }

    impl Model for Qwen {
        fn options(&self) -> &ModelOptions {
            &self.options
        }
    }

    impl StreamingChatModel for Qwen {}

    #[tokio::test]
    async fn test_qwen_chat() {
        let propmt = Prompt::new()
            .system("先分析图片中的所有元素，再回答用户的问题。")
            .message(
                Message::media(Role::User)
                    .image_url("https://qwenlm.github.io/img/logo.png")
                    .text("这是什么")
                    .into(),
            );
        println!("{}", serde_json::to_string(&propmt).unwrap());
        let qwen = Qwen::new();
        let completion = qwen.completion(&propmt).await.unwrap();
        println!("{completion:?}");
    }
}
