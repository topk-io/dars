use std::marker::PhantomData;

use crate::Signature;

pub trait Module {
    type Signature: Signature;

    fn call(
        &self,
        input: <Self::Signature as Signature>::Input,
    ) -> <Self::Signature as Signature>::Output;
}

pub struct Predict<S: Signature> {
    signature: S,
}

impl<S: Signature> Predict<S> {
    pub fn new(signature: S) -> Self {
        println!("signature: {:?}", signature);
        Self { signature }
    }
}

impl<S: Signature> Module for Predict<S> {
    type Signature = S;

    fn call(
        &self,
        input: <Self::Signature as Signature>::Input,
    ) -> <Self::Signature as Signature>::Output {
        println!("input: {:?}", input);
        todo!()
    }
}
