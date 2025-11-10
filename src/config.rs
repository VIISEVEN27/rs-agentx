pub enum ModelConfig {
    OpenAI(OpenAIConfig),
}

pub struct OpenAIConfig {
    pub model: String,
    pub base_url: String,
    pub api_key: String,
}
