use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;

use super::Module;
use crate::{Adapter, Error, LM, Message, MessageContent, Signature, json::JsonAdapter};

pub struct Predict<S: Signature> {
    lm: Arc<dyn LM>,
    adapter: JsonAdapter<S>,
}

impl<S: Signature> Predict<S> {
    pub fn new(lm: Arc<dyn LM>, signature: S) -> Self {
        Self {
            lm,
            adapter: JsonAdapter::new(signature),
        }
    }

    pub fn set_lm(&mut self, lm: Arc<dyn LM>) {
        self.lm = lm;
    }
}

#[async_trait]
impl<S: Signature> Module for Predict<S> {
    type Input = <S as Signature>::Input;
    type Output = <S as Signature>::Output;

    async fn call(&self, input: Self::Input) -> Result<Self::Output, Error> {
        let (messages, schema) = self.adapter.format(input)?;
        for m in &messages {
            println!("{}", m);
        }

        // Call LM with the json schema for the output
        let resp = self.lm.call(messages, schema).await?;

        self.adapter.parse(resp)
    }
}
