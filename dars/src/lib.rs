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

#[derive(Debug, thiserror::Error)]
pub enum Error {}
