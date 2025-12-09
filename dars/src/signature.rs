use std::fmt::Debug;

use schemars::JsonSchema;
use serde::{Serialize, de::DeserializeOwned};

pub trait SignatureInput: Debug + Serialize + JsonSchema + Send + Sync + 'static {}

pub trait SignatureOutput: Debug + DeserializeOwned + JsonSchema + Send + Sync + 'static {}

pub trait Signature
where
    Self: Sized + Debug + Send + Sync + 'static,
{
    type Input: SignatureInput;
    type Output: SignatureOutput;

    /// Returns the instruction for the signature.
    fn instruction(&self) -> &str;

    /// Returns the input schema for the signature.
    fn input_schema(&self) -> schemars::Schema;

    /// Returns the output schema for the signature.
    fn output_schema(&self) -> schemars::Schema;
}
