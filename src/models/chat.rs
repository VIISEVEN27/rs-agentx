use crate::{
    completion::Completion,
    config::ModelConfig,
    models::{
        Model,
        openai::{completion_openai, stream_openai},
    },
    prompt::Prompt,
};

use futures::stream::{Stream, StreamExt};
use std::pin::Pin;

pub trait ChatModel: Model + Send + Sync {
    fn completion(
        &self,
        prompt: &Prompt,
    ) -> impl std::future::Future<Output = anyhow::Result<Completion>> {
        match self.config() {
            ModelConfig::OpenAI(config) => completion_openai(config, prompt),
        }
    }
}

pub trait StreamingChatModel: Model + Send + Sync {
    fn stream<'a>(
        &self,
        prompt: &Prompt,
    ) -> impl Future<Output = anyhow::Result<Pin<Box<dyn Stream<Item = Completion> + Send + Sync>>>>
    {
        match self.config() {
            ModelConfig::OpenAI(config) => stream_openai(config, prompt),
        }
    }

    fn completion(
        &self,
        prompt: &Prompt,
    ) -> impl std::future::Future<Output = anyhow::Result<Completion>> {
        async {
            let mut content = None;
            let mut reasoning_content = None;
            let mut usage = None;
            match self.config() {
                _ => {
                    let mut stream = self.stream(prompt).await?;
                    while let Some(chunk) = stream.next().await {
                        if let Some(content_chunk) = chunk.content {
                            *content.get_or_insert_default() += content_chunk.as_str();
                        }
                        if let Some(reasoning_content_chunk) = chunk.reasoning_content {
                            *reasoning_content.get_or_insert_default() +=
                                reasoning_content_chunk.as_str();
                        }
                        if let Some(usage_chunk) = chunk.usage {
                            usage = Some(usage_chunk);
                        }
                    }
                }
            };
            Ok(Completion {
                content,
                reasoning_content,
                usage,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        config::{ModelConfig, OpenAIConfig},
        message::{Message, Role},
        prompt::Prompt,
    };

    use super::*;

    struct Qwen {
        config: ModelConfig,
    }

    impl Qwen {
        fn new() -> Self {
            Self {
                config: ModelConfig::OpenAI(OpenAIConfig {
                    model: "qwen-vl-plus-2025-08-15".to_string(),
                    base_url: "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions"
                        .to_string(),
                    api_key: "".to_string(),
                }),
            }
        }
    }

    impl Model for Qwen {
        fn config(&self) -> &ModelConfig {
            &self.config
        }
    }

    impl StreamingChatModel for Qwen {}

    #[tokio::test]
    async fn test_qwen_chat() {
        let propmt = Prompt::builder()
            .system("先分析图片中的所有元素，再回答用户的问题。".to_string())
            .message(
                Message::builder()
                    .image_url("https://www.baidu.com/img/bd_logo.png".to_string())
                    .text("这是什么".to_string())
                    .build(Role::User),
            )
            .build();
        println!("{}", serde_json::to_string(&propmt).unwrap());
        let qwen = Qwen::new();
        let completion = qwen.completion(&propmt).await.unwrap();
        println!("{completion:?}");
    }
}
