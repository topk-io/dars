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

    /// Returns output fields for the signature.
    fn output_fields(&self) -> &[Field];

    /// Returns the [`Schema`](schemars::Schema) for a field by name.
    fn field(&self, name: &str) -> Option<&schemars::Schema>;
}
