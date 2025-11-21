pub mod completion;
pub mod message;
pub mod models;
pub mod options;
pub mod prompt;
pub mod usage;

pub use completion::Completion;
pub use message::{Message, Role};
pub use models::{
    Model, Stream,
    chat::{ChatModel, StreamingChatModel},
};
pub use options::{ModelOptions, OpenAIModelOptions};
pub use prompt::Prompt;
pub use usage::Usage;

pub use futures::stream::StreamExt;
