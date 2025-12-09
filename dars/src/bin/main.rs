use dars::{Model, Module, Predict, Signature};
use schemars::schema_for;

#[Model]
struct Step {
    #[field(desc = "foo")]
    id: u16,

    #[field(desc = "The step dependencies")]
    dependencies: Vec<u16>,
}

#[Signature("This is my instruction")]
struct Plan {
    #[input(desc = "The question to answer")]
    question: String,

    #[output(desc = "Plan steps to answer the question")]
    steps: Vec<Step>,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let output_schema = schema_for!(PlanOutput);
    println!("{}", serde_json::to_string_pretty(&output_schema).unwrap());

    let plan = Predict::new(Plan::new());
    println!(
        "plan: {:?}",
        plan.call(PlanInput {
            question: "Hi how are you?".to_string(),
        })
        .await
        .unwrap()
    );
}
