use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ModelOptions {
    OpenAI(OpenAIModelOptions),
    Whatever,
}

impl ModelOptions {
    pub fn openai() -> OpenAIModelOptions {
        OpenAIModelOptions::new()
    }
}

impl Default for ModelOptions {
    fn default() -> Self {
        Self::Whatever
    }
}

pub enum BorrowedModelOptions<'a> {
    OpenAI(BorrowedOpenAIModelOptions<'a>),
    Whatever,
}

impl ModelOptions {
    pub fn borrow(&self) -> BorrowedModelOptions<'_> {
        match self {
            Self::OpenAI(options) => options.borrow().into(),
            Self::Whatever => BorrowedModelOptions::Whatever,
        }
    }

    pub fn merge<'a>(&'a self, other: &'a Self) -> BorrowedModelOptions<'a> {
        match (self, other) {
            (Self::OpenAI(options), Self::OpenAI(other_options)) => {
                options.merge(other_options).into()
            }
            (_, Self::Whatever) => self.borrow(),
            (Self::Whatever, _) => other.borrow(),
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
}

impl From<OpenAIModelOptions> for ModelOptions {
    fn from(options: OpenAIModelOptions) -> Self {
        Self::OpenAI(options)
    }
}

pub struct BorrowedOpenAIModelOptions<'a> {
    pub model: Option<&'a str>,
    pub base_url: Option<&'a str>,
    pub api_key: Option<&'a str>,
}

impl OpenAIModelOptions {
    pub fn borrow(&self) -> BorrowedOpenAIModelOptions<'_> {
        BorrowedOpenAIModelOptions {
            model: self.model.as_deref(),
            base_url: self.base_url.as_deref(),
            api_key: self.api_key.as_deref(),
        }
    }

    pub fn merge<'a>(&'a self, other: &'a Self) -> BorrowedOpenAIModelOptions<'a> {
        BorrowedOpenAIModelOptions {
            model: other.model.as_deref().or(self.model.as_deref()),
            base_url: other.base_url.as_deref().or(self.base_url.as_deref()),
            api_key: other.api_key.as_deref().or(self.api_key.as_deref()),
        }
    }
}

impl<'a> From<BorrowedOpenAIModelOptions<'a>> for BorrowedModelOptions<'a> {
    fn from(options: BorrowedOpenAIModelOptions<'a>) -> Self {
        Self::OpenAI(options)
    }
}
