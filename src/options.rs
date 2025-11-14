use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum ModelOptions {
    OpenAI(OpenAIModelOptions),
}

#[derive(Clone, Debug)]
pub struct OpenAIModelOptions {
    pub base_url: String,
    pub model: Option<String>,
    pub api_key: Option<String>,
}

impl OpenAIModelOptions {
    pub fn new<T: Display>(base_url: T) -> Self {
        Self {
            base_url: base_url.to_string(),
            model: None,
            api_key: None,
        }
    }

    pub fn model<T: Display>(mut self, model: T) -> Self {
        self.model = Some(model.to_string());
        self
    }

    pub fn api_key<T: Display>(mut self, api_key: T) -> Self {
        self.api_key = Some(api_key.to_string());
        self
    }
}

impl From<OpenAIModelOptions> for ModelOptions {
    fn from(options: OpenAIModelOptions) -> Self {
        Self::OpenAI(options)
    }
}
