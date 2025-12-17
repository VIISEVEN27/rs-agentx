use anyhow::Ok;
use async_trait::async_trait;

use crate::{options::BorrowedModelOptions, Completion, Model, ModelOptions, Prompt, Stream};

mod openai;

#[async_trait]
pub trait ChatModel: Model {
    async fn completion(
        &self,
        prompt: &Prompt,
        options: ModelOptions,
    ) -> anyhow::Result<Completion> {
        let options = self.options().merge(&options);
        match options {
            BorrowedModelOptions::OpenAI(options) => {
                Ok(openai::completion(prompt, options).await?.into())
            }
            _ => panic!("Unsupported type of 'ModelOptions'"),
        }
    }

    async fn text_completion(
        &self,
        prompt: &Prompt,
        options: ModelOptions,
    ) -> anyhow::Result<String> {
        let options = self.options().merge(&options);
        match options {
            BorrowedModelOptions::OpenAI(options) => {
                Ok(openai::completion(prompt, options).await?.to_string())
            }
            _ => panic!("Unsupported type of 'ModelOptions'"),
        }
    }
}

#[async_trait]
pub trait StreamingChatModel: Model {
    async fn stream(
        &self,
        prompt: &Prompt,
        options: ModelOptions,
    ) -> anyhow::Result<Stream<Completion>> {
        let options = self.options().merge(&options);
        match options {
            BorrowedModelOptions::OpenAI(options) => {
                Ok(openai::stream(prompt, options).await?.into())
            }
            _ => panic!("Unsupported type of 'ModelOptions'"),
        }
    }

    async fn text_stream(
        &self,
        prompt: &Prompt,
        options: ModelOptions,
    ) -> anyhow::Result<Stream<String>> {
        let options = self.options().merge(&options);
        match options {
            BorrowedModelOptions::OpenAI(options) => {
                Ok(openai::stream(prompt, options).await?.into())
            }
            _ => panic!("Unsupported type of 'ModelOptions'"),
        }
    }

    async fn completion(
        &self,
        prompt: &Prompt,
        options: ModelOptions,
    ) -> anyhow::Result<Completion> {
        let options = self.options().merge(&options);
        match options {
            BorrowedModelOptions::OpenAI(options) => {
                Ok(openai::stream(prompt, options).await?.collect().await)
            }
            _ => panic!("Unsupported type of 'ModelOptions'"),
        }
    }

    async fn text_completion(
        &self,
        prompt: &Prompt,
        options: ModelOptions,
    ) -> anyhow::Result<String> {
        Ok(self.text_stream(prompt, options).await?.collect().await)
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
        let completion = qwen
            .completion(&propmt, ModelOptions::default())
            .await
            .unwrap();
        println!("{completion:?}");
    }
}
