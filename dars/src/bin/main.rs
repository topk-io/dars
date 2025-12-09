use std::sync::Arc;

use async_openai::config::OpenAIConfig;
use dars::{
    Model, Module, Predict, Signature,
    openai::{ModelConfig, OpenAILM},
};

#[Model]
struct Step {
    #[field(desc = "foobarbaz")]
    id: u16,

    #[field(desc = "The step dependencies")]
    dependencies: Vec<u16>,
}

#[Signature("This is my instruction")]
// #[Signature]
struct Plan {
    #[input]
    question: String,

    #[input]
    previous_steps: Vec<usize>,

    #[output(desc = "Plan steps to answer the question")]
    steps: Vec<Step>,
}

#[tokio::main]
async fn main() {
    let sig = Plan::new();
    println!("input: {:?}", sig.input_schema());
    println!("output: {:?}", sig.output_schema());
    println!("input fields: {:?}", sig.input_fields());
    println!("output fields: {:?}", sig.output_fields());

    let lm = Arc::new(OpenAILM::new(
        OpenAIConfig::new(),
        ModelConfig::model("gpt-41-mini"),
    ));

    let plan = Predict::new(lm, sig);

    println!(
        "plan: {:?}",
        plan.call(PlanInput {
            question: "Hi how are you?".to_string(),
            previous_steps: vec![1, 2, 3],
        })
        .await
        .unwrap()
    );
}
