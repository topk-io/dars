use std::sync::Arc;

use async_trait::async_trait;

use super::Module;
use crate::{Error, LM, Message, Signature};

pub struct Predict<S: Signature> {
    lm: Arc<dyn LM>,
    signature: S,
}

impl<S: Signature> Predict<S> {
    pub fn new(lm: Arc<dyn LM>, signature: S) -> Self {
        Self { lm, signature }
    }
}

#[async_trait]
impl<S: Signature> Module for Predict<S> {
    type Input = <S as Signature>::Input;
    type Output = <S as Signature>::Output;

    async fn call(&self, input: Self::Input) -> Result<Self::Output, Error> {
        let messages = vec![Message::System {
            instruction: self.signature.instruction().to_string(),
        }];

        // Call LM with the json schema for the output
        let resp = self
            .lm
            .call(messages, Some(self.signature.output_schema().clone()))
            .await?;

        Ok(serde_json::from_str(&resp)?)
    }
}
