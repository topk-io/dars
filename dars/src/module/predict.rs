use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;

use super::Module;
use crate::{Error, LM, Message, MessageContent, Signature};

pub struct Predict<S: Signature> {
    lm: Arc<dyn LM>,
    signature: S,
}

impl<S: Signature> Predict<S> {
    pub fn new(lm: Arc<dyn LM>, signature: S) -> Self {
        Self { lm, signature }
    }

    pub fn set_lm(&mut self, lm: Arc<dyn LM>) {
        self.lm = lm;
    }

    fn format_system_message(&self) -> Message {
        let mut buf = String::new();
        // Input fields
        buf += "Your input fields are:\n";
        for (i, f) in self.signature.input_fields().iter().enumerate() {
            buf += &format!(
                "{}. `{}`: {}\n",
                i + 1,
                f.name,
                f.description.unwrap_or_default()
            );
        }
        // Output fields
        buf += "\nYour output fields are:\n";
        for (i, f) in self.signature.output_fields().iter().enumerate() {
            buf += &format!(
                "{}. `{}`: {}\n",
                i + 1,
                f.name,
                f.description.unwrap_or_default()
            );
        }
        buf += "All interactions will be structured in the following way, with the appropriate values filled in.\n";
        // Input structure
        buf += "\nInputs will have the following structure:\n";
        for f in self.signature.input_fields() {
            buf += &format!("\n[[ ## {} ## ]]\n{{{}}}\n", f.name, f.name)
        }
        // Output structure
        buf += "\nOutputs will be a JSON object with the following fields.\n";
        buf += "{\n";
        for f in self.signature.output_fields() {
            buf += &format!("\t\"{}\": \"{{{}}}\",\n", f.name, f.name);
        }
        buf += "\n}";
        buf += "\nIn adhering to this structure, your objective is:\n";
        if self.signature.instruction().is_empty() {
            buf += "Given the fields ";
            for f in self.signature.input_fields() {
                buf += &format!("`{}`, ", f.name);
            }
            buf += "produce the fields ";
            for f in self.signature.output_fields() {
                buf += &format!("`{}`, ", f.name);
            }
            buf += ".";
        } else {
            buf += &self.signature.instruction();
        }

        Message::System { instruction: buf }
    }

    fn format_input(&self, input: S::Input) -> Result<Message, Error> {
        match serde_json::to_value(input)? {
            Value::Object(kv) => {
                let mut buf = String::new();
                for f in self.signature.input_fields() {
                    match kv.get(f.name) {
                        Some(value) => buf += &format!("[[ ## {} ## ]]\n{}\n\n", f.name, value),
                        None => buf += &format!("[[ ## {} ## ]]\n\n", f.name),
                    }
                }
                Ok(Message::User {
                    content: MessageContent::Text { text: buf },
                })
            }
            _ => unreachable!(),
        }
    }

    fn parse_output(&self, output: String) -> Result<S::Output, Error> {
        Ok(serde_json::from_str(&output)?)
    }
}

#[async_trait]
impl<S: Signature> Module for Predict<S> {
    type Input = <S as Signature>::Input;
    type Output = <S as Signature>::Output;

    async fn call(&self, input: Self::Input) -> Result<Self::Output, Error> {
        let messages = vec![
            // System message
            self.format_system_message(),
            // Input message
            self.format_input(input)?,
        ];
        for m in &messages {
            println!("{}", m);
        }

        // Call LM with the json schema for the output
        let resp = self
            .lm
            .call(messages, Some(self.signature.output_schema().clone()))
            .await?;

        self.parse_output(resp)
    }
}
