use std::fmt::Debug;

use crate::{Field, model::Model};

pub trait Signature
where
    Self: Sized + Debug + Send + Sync + 'static,
{
    type Input: Model;
    type Output: Model;

    /// Returns the instruction for the signature.
    fn instruction(&self) -> &str;

    /// Returns input fields for the signature.
    fn input_fields(&self) -> &[Field];

    /// Returns the input schema for the signature.
    fn input_schema(&self) -> &schemars::Schema;

    /// Returns output fields for the signature.
    fn output_fields(&self) -> &[Field];

    /// Returns the output schema for the signature.
    fn output_schema(&self) -> &schemars::Schema;
}
