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

mod lm;
pub use lm::*;

#[derive(Debug, Clone)]
pub struct Field {
    pub name: &'static str,
    pub description: Option<&'static str>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("serde_json: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[cfg(feature = "openai")]
    #[error("OpenAI: {0}")]
    OpenAI(#[from] async_openai::error::OpenAIError),
}
