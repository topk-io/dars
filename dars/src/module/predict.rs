use async_trait::async_trait;

use super::Module;
use crate::{Error, Signature};

pub struct Predict<S: Signature> {
    signature: S,
}

impl<S: Signature> Predict<S> {
    pub fn new(signature: S) -> Self {
        println!("predict signature: {:?}", signature);
        Self { signature }
    }
}

#[async_trait]
impl<S: Signature> Module for Predict<S> {
    type Input = <S as Signature>::Input;
    type Output = <S as Signature>::Output;

    async fn call(&self, input: Self::Input) -> Result<Self::Output, Error> {
        println!("input: {:?}", input);
        todo!()
    }
}
