use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}
