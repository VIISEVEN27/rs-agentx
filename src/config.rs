use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct OpenAIConfig {
    pub model: String,
    pub base_url: String,
    pub api_key: String,
}

pub enum ModelConfig {
    OpenAI(OpenAIConfig),
}
