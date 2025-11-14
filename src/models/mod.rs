use crate::options::ModelOptions;

pub mod chat;
mod openai;

pub trait Model {
    fn options(&self) -> &ModelOptions;
}
