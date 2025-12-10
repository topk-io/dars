use dars::{Field, Model, Signature};

#[test]
fn test_empty_signature() {
    #[Signature]
    struct EmptySignature {}

    let sig = EmptySignature::new();
    assert_eq!(sig.instruction(), "");
    assert_eq!(sig.input_fields(), &[]);
    assert_eq!(sig.output_fields(), &[]);
}

#[test]
fn test_signature_with_instruction() {
    #[Signature("You are a general purpose AI assistant.")]
    struct GeneralAssistant {}

    let sig = GeneralAssistant::new();
    assert_eq!(sig.instruction(), "You are a general purpose AI assistant.");
    assert_eq!(sig.input_fields(), &[]);
    assert_eq!(sig.output_fields(), &[]);
}

#[test]
fn test_signature_with_input_and_output() {
    #[Signature("Signature instruction")]
    struct SignatureWithInputAndOutput {
        #[input]
        input: String,

        #[input(desc = "Input description")]
        input_with_description: String,

        #[output]
        output: String,

        #[output(desc = "Output description")]
        output_with_description: String,
    }

    let sig = SignatureWithInputAndOutput::new();
    assert_eq!(sig.instruction(), "Signature instruction");
    assert_eq!(
        sig.input_fields(),
        &[
            Field {
                name: "input",
                description: None
            },
            Field {
                name: "input_with_description",
                description: Some("Input description")
            }
        ]
    );
    assert_eq!(
        sig.output_fields(),
        &[
            Field {
                name: "output",
                description: None
            },
            Field {
                name: "output_with_description",
                description: Some("Output description")
            }
        ]
    );
}

#[test]
fn test_signature_generates_input_output_models() {
    #[Signature]
    struct Sig {
        #[input]
        input: String,

        #[input(desc = "Input description")]
        input_with_description: String,

        #[output]
        output: String,

        #[output(desc = "Output description")]
        output_with_description: String,
    }

    assert_eq!(
        SigInput::fields(),
        &[
            Field {
                name: "input",
                description: None
            },
            Field {
                name: "input_with_description",
                description: Some("Input description")
            }
        ]
    );

    assert_eq!(
        SigOutput::fields(),
        &[
            Field {
                name: "output",
                description: None
            },
            Field {
                name: "output_with_description",
                description: Some("Output description")
            }
        ]
    );
}
