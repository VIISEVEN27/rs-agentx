use std::fmt::Display;

use serde::Serialize;

use crate::message::{Message, Role};

#[derive(Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Prompt(Vec<Message>);

impl Prompt {
    pub fn new() -> Self {
        Prompt(Vec::new())
    }

    pub fn create<T: Display>(content: T) -> Self {
        Self(vec![Message::text(Role::User, content.to_string())])
    }

    pub fn message(mut self, message: Message) -> Self {
        self.0.push(message);
        self
    }

    pub fn system<T: Display>(self, content: T) -> Self {
        self.message(Message::text(Role::System, content))
    }

    pub fn user<T: Display>(self, content: T) -> Self {
        self.message(Message::text(Role::User, content))
    }

    pub fn assistant<T: Display>(self, content: T) -> Self {
        self.message(Message::text(Role::Assistant, content))
    }
}

impl From<Vec<Message>> for Prompt {
    fn from(messages: Vec<Message>) -> Self {
        Prompt(messages)
    }
}
