use std::fmt::Debug;

use schemars::JsonSchema;
use serde::{Serialize, de::DeserializeOwned};

pub trait Model
where
    Self: Sized + Debug + Serialize + DeserializeOwned + JsonSchema + Send + Sync + 'static,
{
    /// Returns fields in ths model. The returned tuple is `(name, description)`.
    fn fields() -> &'static [(&'static str, Option<&'static str>)];
}
