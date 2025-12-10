use schemars::Schema;

use crate::{Error, Signature, lm::Message};

pub mod json;

pub trait Adapter<S: Signature>: Send + Sync + 'static {
    /// Format the input as a list of chat messages with an optional json schema
    /// for the output.
    fn format(&self, input: S::Input) -> Result<(Vec<Message>, Option<Schema>), Error>;

    /// Parse the output as the signature output type.
    fn parse(&self, output: String) -> Result<S::Output, Error>;
}
