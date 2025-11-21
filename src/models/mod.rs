use std::{
    ops::{Deref, DerefMut},
    pin::Pin,
};

use futures::StreamExt;

use crate::{Completion, options::ModelOptions};

pub mod chat;

pub trait Model: Send + Sync + 'static {
    fn options(&self) -> &ModelOptions;
}

pub struct Stream<T>(InnerStream<T>);
type InnerStream<T> = Pin<Box<dyn futures::stream::Stream<Item = T> + Send + Sync + 'static>>;

impl<T> Stream<T> {
    pub fn new(stream: InnerStream<T>) -> Self {
        Self(stream)
    }

    pub fn into_inner(self) -> InnerStream<T> {
        self.0
    }
}

impl<T, S: futures::stream::Stream<Item = T> + Send + Sync + 'static> From<S> for Stream<T> {
    fn from(stream: S) -> Self {
        Self::new(Box::pin(stream))
    }
}

impl<T> From<Stream<T>> for InnerStream<T> {
    fn from(stream: Stream<T>) -> Self {
        stream.into_inner()
    }
}

impl<T> Deref for Stream<T> {
    type Target = InnerStream<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Stream<T> {
    fn deref_mut(&mut self) -> &mut InnerStream<T> {
        &mut self.0
    }
}

impl Stream<Completion> {
    pub async fn collect(mut self) -> Completion {
        let mut content = None;
        let mut reasoning_content = None;
        let mut usage = None;
        while let Some(item) = self.next().await {
            if let Some(content_chunk) = item.content {
                *content.get_or_insert_default() += content_chunk.as_str();
            }
            if let Some(reasoning_content_chunk) = item.reasoning_content {
                *reasoning_content.get_or_insert_default() += reasoning_content_chunk.as_str();
            }
            if let Some(usage_chunk) = item.usage {
                usage = Some(usage_chunk);
            }
        }
        Completion::new(content, reasoning_content, usage)
    }
}

impl Stream<String> {
    pub async fn collect(self) -> String {
        self.into_inner()
            .fold(String::new(), async |acc, item| acc + &item.to_string())
            .await
    }
}
