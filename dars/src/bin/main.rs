use dars::{Model, Module, Predict, Signature};

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

    let step = Step {
        id: 1,
        dependencies: vec![],
    };
    println!("fields: {:?}", Step::fields());

    let plan = Predict::new(sig);

    println!(
        "plan: {:?}",
        plan.call(PlanInput {
            question: "Hi how are you?".to_string(),
        })
        .await
        .unwrap()
    );
}
