use serde::{Deserialize, Serialize};

use crate::message::{Message, Role};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(transparent)]
pub struct Prompt(Vec<Message>);

impl Prompt {
    pub fn new() -> Self {
        Prompt(Vec::new())
    }

    pub fn create<T: AsRef<str>>(content: T) -> Self {
        Self(vec![Message::text(Role::User, content)])
    }

    pub fn message(mut self, message: Message) -> Self {
        self.0.push(message);
        self
    }

    pub fn system<T: AsRef<str>>(self, content: T) -> Self {
        self.message(Message::text(Role::System, content))
    }

    pub fn user<T: AsRef<str>>(self, content: T) -> Self {
        self.message(Message::text(Role::User, content))
    }

    pub fn assistant<T: AsRef<str>>(self, content: T) -> Self {
        self.message(Message::text(Role::Assistant, content))
    }
}

impl From<Vec<Message>> for Prompt {
    fn from(messages: Vec<Message>) -> Self {
        Prompt(messages)
    }
}
