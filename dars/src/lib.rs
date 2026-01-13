// Re-export dependencies
pub use schemars;
pub use serde;

// Export macros
pub use dars_macros::*;

mod signature;
pub use signature::*;

pub mod model;
pub use model::*;

mod module;
pub use module::*;

mod image;
pub use image::Image;

pub mod adapter;
pub mod lm;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub name: &'static str,
    pub description: Option<&'static str>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("serde: {0}")]
    SerdeJson(serde_json::Error),

    #[cfg(feature = "openai")]
    #[error("OpenAI: {0}")]
    OpenAI(#[from] async_openai::error::OpenAIError),

    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("model call failed: {0}")]
    ModelCall(String),
}
