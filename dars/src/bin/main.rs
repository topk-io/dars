use dars::Signature;

#[Signature("This is my instruction")]
struct Plan {
    #[input(desc = "The question to answer")]
    question: String,

    #[input(desc = "foo bar")]
    olala: String,

    #[output(desc = "Plan steps to answer the question")]
    steps: Vec<Step>,
}

fn main() {
    println!("Hello, world!");
}
