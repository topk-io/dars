use async_trait::async_trait;

mod predict;
pub use predict::Predict;

use crate::Error;

#[async_trait]
pub trait Module: Send + Sync + 'static {
    type Input;
    type Output;

    async fn call(&self, input: Self::Input) -> Result<Self::Output, Error>;
}
