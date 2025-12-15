use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ModelOptions {
    OpenAI(OpenAIModelOptions),
}

impl ModelOptions {
    pub fn openai() -> OpenAIModelOptions {
        OpenAIModelOptions::new()
    }

    pub fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Self::OpenAI(options), Self::OpenAI(other_options)) => {
                options.merge(other_options).into()
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OpenAIModelOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub api_key: Option<String>,
}

impl OpenAIModelOptions {
    pub fn new() -> Self {
        Self {
            base_url: None,
            model: None,
            api_key: None,
        }
    }

    pub fn model<T: AsRef<str>>(mut self, model: T) -> Self {
        self.model = Some(model.as_ref().to_owned());
        self
    }

    pub fn base_url<T: AsRef<str>>(mut self, base_url: T) -> Self {
        self.base_url = Some(base_url.as_ref().to_owned());
        self
    }

    pub fn api_key<T: AsRef<str>>(mut self, api_key: T) -> Self {
        self.api_key = Some(api_key.as_ref().to_owned());
        self
    }

    pub fn merge(self, other: Self) -> Self {
        let Self {
            model,
            base_url,
            api_key,
        } = other;
        Self {
            model: model.or(self.model),
            base_url: base_url.or(self.base_url),
            api_key: api_key.or(self.api_key),
        }
    }
}

impl From<OpenAIModelOptions> for ModelOptions {
    fn from(options: OpenAIModelOptions) -> Self {
        Self::OpenAI(options)
    }
}
