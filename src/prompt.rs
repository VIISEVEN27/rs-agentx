use serde::Serialize;

use crate::message::Message;
use std::ops::{Deref, DerefMut};

#[derive(Serialize, Debug)]
#[serde(transparent)]
pub struct Prompt(Vec<Message>);

impl Deref for Prompt {
    type Target = Vec<Message>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Prompt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Prompt> for Vec<Message> {
    fn from(prompt: Prompt) -> Vec<Message> {
        prompt.0
    }
}

impl<'a> From<&'a Prompt> for &'a Vec<Message> {
    fn from(prompt: &'a Prompt) -> &'a Vec<Message> {
        &prompt.0
    }
}

impl Prompt {
    pub fn new(messages: Vec<Message>) -> Self {
        Prompt(messages)
    }

    pub fn user(content: String) -> Self {
        Prompt::new(vec![Message::user(content)])
    }

    pub fn builder() -> PromptBuilder {
        PromptBuilder::new()
    }
}

pub struct PromptBuilder(Vec<Message>);

impl PromptBuilder {
    pub fn new() -> Self {
        PromptBuilder(Vec::new())
    }

    pub fn message(mut self, message: Message) -> Self {
        self.0.push(message);
        self
    }

    pub fn system(self, content: String) -> Self {
        self.message(Message::system(content))
    }

    pub fn user(self, content: String) -> Self {
        self.message(Message::user(content))
    }

    pub fn assistant(self, content: String) -> Self {
        self.message(Message::assistant(content))
    }

    pub fn build(self) -> Prompt {
        Prompt::new(self.0)
    }
}
