use std::sync::Arc;

use async_trait::async_trait;
use schemars::Schema;
use serde_json::json;

use dars::{
    Error, Module, Predict, Signature,
    lm::{LM, Message},
};

struct FixedLM {
    resp: serde_json::Value,
}

impl FixedLM {
    fn new(resp: serde_json::Value) -> Self {
        Self { resp }
    }
}

#[async_trait]
impl LM for FixedLM {
    async fn call(&self, _input: Vec<Message>, _schema: Option<Schema>) -> Result<String, Error> {
        Ok(serde_json::to_string(&self.resp)?)
    }
}

#[Signature]
struct Sig {
    #[input(desc = "The question to answer")]
    question: String,

    #[output(desc = "The answer to the question")]
    answer: String,

    #[output(desc = "The confidence in the answer")]
    confidence: f32,
}

#[tokio::test]
async fn test_predict_base() {
    let lm = Arc::new(FixedLM::new(json!({
        "answer": "output value",
        "confidence": 0.95
    })));

    let predict = Predict::new(lm, Sig::new());
    let output = predict
        .call(SigInput {
            question: "input value".to_string(),
        })
        .await
        .unwrap();
    assert_eq!(output.answer, "output value");
    assert_eq!(output.confidence, 0.95);
}

#[tokio::test]
async fn test_predict_with_invalid_output() {
    let lm = Arc::new(FixedLM::new(json!({
        "answer": "output value",
        "confidence": "foobar"
    })));

    let predict = Predict::new(lm, Sig::new());
    let output = predict
        .call(SigInput {
            question: "input value".to_string(),
        })
        .await
        .expect_err("should error");

    assert!(matches!(output, Error::SerdeJson(_)));
}
