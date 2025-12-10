# Declarative Agents .rs
DSPy is great. Rust is also great and much safer in production. This crate provides a way to build
declarative agents natively in Rust. It closely follows DSPy API to allow for easy portability of 
research/experimental code.

## Example
```rust
use std::collections::HashMap;
use std::sync::Arc;

use async_openai::config::OpenAIConfig;
use dars::lm::openai::{ModelConfig, OpenAILM};
use dars::*;

#[Signature("Extract structured information from text.")]
struct ExtractInfo {
    #[input]
    text: String,

    #[output]
    title: String,

    #[output]
    headings: Vec<String>,

    #[output(desc = "List of entities and their metadata")]
    entities: Vec<HashMap<String, String>>,
}

#[tokio::main]
async fn main() {
    let lm = Arc::new(OpenAILM::new(
        OpenAIConfig::default(),
        ModelConfig::model("gpt-4o-mini"),
    ));
    let module = Predict::new(lm, ExtractInfo::new());

    let output = module
        .call(ExtractInfoInput {
            text: "Apple Inc. announced its latest iPhone 14 today.
                The CEO, Tim Cook, highlighted its new features in a press release."
                .into(),
        })
        .await
        .unwrap();

    println!("{:?}", output);
}
```