use crate::config::ModelConfig;

pub mod chat;
mod openai;

pub trait Model {
    fn config(&self) -> &ModelConfig;
}
