use std::ops::{Deref, DerefMut};

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

    pub fn is_media(self) -> bool {
        for message in &self.0 {
            if let Message::Media(_) = message {
                return true;
            }
        }
        false
    }
}

impl From<Vec<Message>> for Prompt {
    fn from(messages: Vec<Message>) -> Self {
        Prompt(messages)
    }
}

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

impl IntoIterator for Prompt {
    type Item = Message;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
