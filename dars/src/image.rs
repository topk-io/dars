use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError};

#[derive(Debug, Clone, JsonSchema)]
pub struct Image {
    #[schemars(description = "Image url or encoded image data")]
    pub url: String,
}

impl Serialize for Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("<dars-img>{}</dars-img>", self.url))
    }
}

impl<'de> Deserialize<'de> for Image {
    fn deserialize<D>(_de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Err(D::Error::custom("Deserializing image is not supported"))
    }
}
