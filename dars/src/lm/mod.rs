use std::fmt::Display;

use async_trait::async_trait;
use schemars::Schema;

use crate::Error;

#[cfg(feature = "openai")]
pub mod openai;

#[async_trait]
pub trait LM
where
    Self: Send + Sync + 'static,
{
    /// Call the LM with the given input messages and an optional json schema for the output.
    async fn call(&self, message: Vec<Message>, schema: Option<Schema>) -> Result<String, Error>;
}

#[derive(Debug, Clone)]
pub enum Message {
    System { instruction: String },
    User { content: Vec<MessageContent> },
    Assistant { content: MessageContent },
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::System { instruction } => write!(f, "System:\n{}", instruction),
            Message::User { content } => {
                write!(f, "User:\n")?;
                for c in content.iter() {
                    write!(f, "{}\n", c)?;
                }
                Ok(())
            }
            Message::Assistant { content } => write!(f, "Assistant:\n{}", content),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageContent {
    Text { text: String },
    Image { url: String },
}

impl Display for MessageContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageContent::Text { text } => write!(f, "{}", text),
            MessageContent::Image { url } => write!(f, "<image len={}>", url.len()),
        }
    }
}
