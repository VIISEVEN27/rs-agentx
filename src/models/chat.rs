use crate::{
    completion::Completion,
    models::{
        Model,
        openai::{completion_openai, stream_openai},
    },
    options::ModelOptions,
    prompt::Prompt,
};

use futures::stream::{Stream, StreamExt};
use std::pin::Pin;

pub trait ChatModel: Model + Send + Sync {
    fn completion_with(
        &self,
        prompt: &Prompt,
        options: &ModelOptions,
    ) -> impl Future<Output = anyhow::Result<Completion>> {
        match options {
            ModelOptions::OpenAI(options) => completion_openai(prompt, options),
        }
    }

    fn completion(&self, prompt: &Prompt) -> impl Future<Output = anyhow::Result<Completion>> {
        self.completion_with(prompt, self.options())
    }
}

pub trait StreamingChatModel: Model + Send + Sync {
    fn stream_with<'a>(
        &self,
        prompt: &Prompt,
        options: &ModelOptions,
    ) -> impl Future<Output = anyhow::Result<Pin<Box<dyn Stream<Item = Completion> + Send + Sync>>>>
    {
        match options {
            ModelOptions::OpenAI(options) => stream_openai(prompt, options),
        }
    }

    fn stream<'a>(
        &self,
        prompt: &Prompt,
    ) -> impl Future<Output = anyhow::Result<Pin<Box<dyn Stream<Item = Completion> + Send + Sync>>>>
    {
        self.stream_with(prompt, self.options())
    }

    fn completion_with(
        &self,
        prompt: &Prompt,
        options: &ModelOptions,
    ) -> impl Future<Output = anyhow::Result<Completion>> {
        async {
            let mut content = None;
            let mut reasoning_content = None;
            let mut usage = None;
            let mut stream = self.stream_with(prompt, options).await?;
            while let Some(chunk) = stream.next().await {
                if let Some(content_chunk) = chunk.content {
                    *content.get_or_insert_default() += content_chunk.as_str();
                }
                if let Some(reasoning_content_chunk) = chunk.reasoning_content {
                    *reasoning_content.get_or_insert_default() += reasoning_content_chunk.as_str();
                }
                if let Some(usage_chunk) = chunk.usage {
                    usage = Some(usage_chunk);
                }
            }
            Ok(Completion {
                content,
                reasoning_content,
                usage,
            })
        }
    }

    fn completion(&self, prompt: &Prompt) -> impl Future<Output = anyhow::Result<Completion>> {
        self.completion_with(prompt, self.options())
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
                options: OpenAIModelOptions::new(
                    "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions",
                )
                .model("qwen-vl-plus-2025-08-15")
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
                    .image_url("https://www.baidu.com/img/bd_logo.png")
                    .text("这是什么")
                    .into(),
            );
        println!("{}", serde_json::to_string(&propmt).unwrap());
        let qwen = Qwen::new();
        let completion = qwen.completion(&propmt).await.unwrap();
        println!("{completion:?}");
    }
}
