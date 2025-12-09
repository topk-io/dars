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
    async fn call(&self, message: Vec<Message>, schema: Option<Schema>) -> Result<String, Error>;
}

#[derive(Debug, Clone)]
pub enum Message {
    System { instruction: String },
    User { content: MessageContent },
    Assistant { content: MessageContent },
}

#[derive(Debug, Clone)]
pub enum MessageContent {
    Text { text: String },
    Image { url: String },
}
