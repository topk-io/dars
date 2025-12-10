use schemars::{Schema, schema_for};
use serde_json::Value;

use super::Adapter;
use crate::{
    Error, Signature,
    lm::{Message, MessageContent},
};

pub struct JsonAdapter<S: Signature> {
    signature: S,
}

impl<S: Signature> Adapter<S> for JsonAdapter<S> {
    fn format(&self, input: S::Input) -> Result<(Vec<Message>, Option<Schema>), Error> {
        let messages = vec![self.format_system_message(), self.format_input(input)?];
        Ok((messages, Some(schema_for!(S::Output))))
    }

    fn parse(&self, output: String) -> Result<S::Output, Error> {
        Ok(serde_json::from_str(&output)?)
    }
}

impl<S: Signature> JsonAdapter<S> {
    pub fn new(signature: S) -> Self {
        Self { signature }
    }

    fn format_system_message(&self) -> Message {
        let mut buf = String::new();
        // Input fields
        buf += "Your input fields are:\n";
        for (i, f) in self.signature.input_fields().iter().enumerate() {
            let fty = self
                .signature
                .field(f.name)
                .expect("Field not found in schema")
                .as_value();
            buf += &format!("{}. `{}` (", i + 1, f.name);
            fmt_type(fty, &mut buf);
            buf += &format!("): {}\n", f.description.unwrap_or_default());
        }

        // Output fields
        buf += "\nYour output fields are:\n";
        for (i, f) in self.signature.output_fields().iter().enumerate() {
            let fty = self
                .signature
                .field(f.name)
                .expect("Field not found in schema")
                .as_value();
            buf += &format!("{}. `{}` (", i + 1, f.name);
            fmt_type(fty, &mut buf);
            buf += &format!("): {}\n", f.description.unwrap_or_default());
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
        for (i, f) in self.signature.output_fields().iter().enumerate() {
            buf += &format!("\t\"{}\": \"{{{}}}", f.name, f.name);
            if let Some(schema) = self.signature.field(f.name) {
                buf += " # note: the value you produce must adhere to the JSON schema: ";
                buf += &serde_json::to_string(schema).unwrap();
            }
            buf += "\"";
            if i + 1 < self.signature.output_fields().len() {
                buf += ",\n";
            }
        }
        buf += "\n}";

        // Instruction
        buf += "\nIn adhering to this structure, your objective is:\n";
        if self.signature.instruction().is_empty() {
            buf += "Given the fields ";
            for (i, f) in self.signature.input_fields().iter().enumerate() {
                buf += &format!("`{}`", f.name);
                if i + 1 < self.signature.input_fields().len() {
                    buf += ", ";
                }
            }
            buf += ", produce the fields ";
            for (i, f) in self.signature.output_fields().iter().enumerate() {
                buf += &format!("`{}`", f.name);
                if i + 1 < self.signature.output_fields().len() {
                    buf += ", ";
                }
            }
            buf += ".";
        } else {
            buf += &self.signature.instruction().trim();
        }

        Message::System { instruction: buf }
    }

    fn format_input(&self, input: S::Input) -> Result<Message, Error> {
        match serde_json::to_value(input)? {
            Value::Object(kv) => {
                let mut buf = String::new();
                for (i, f) in self.signature.input_fields().iter().enumerate() {
                    match kv.get(f.name) {
                        Some(value) => buf += &format!("[[ ## {} ## ]]\n{}", f.name, value),
                        None => buf += &format!("[[ ## {} ## ]]", f.name),
                    }
                    if i + 1 < self.signature.input_fields().len() {
                        buf += "\n\n";
                    }
                }
                Ok(Message::User {
                    content: MessageContent::Text { text: buf },
                })
            }
            _ => unreachable!(),
        }
    }
}

fn fmt_type(ty: &Value, buf: &mut String) {
    if let Some(vty) = ty.get("type") {
        match vty.as_str().unwrap() {
            "null" => buf.push_str("null"),
            "boolean" => buf.push_str("boolean"),
            "string" => buf.push_str("string"),
            "integer" => buf.push_str("integer"),
            "number" => buf.push_str("number"),
            "array" => {
                buf.push_str("list[");
                if let Some(items) = ty.get("items") {
                    if items.get("type").is_some() {
                        fmt_type(items, buf);
                    } else if let Some(tref) = items.get("$ref") {
                        let ty = tref.as_str().unwrap().split("/").last().unwrap();
                        buf.push_str(ty);
                    }
                }
                buf.push_str("]");
            }
            "object" => {
                buf.push('{');
                let properties = ty.get("properties").map(|v| v.as_object()).flatten();
                if let Some(properties) = properties {
                    for (i, (name, value)) in properties.iter().enumerate() {
                        buf.push_str(name);
                        buf.push_str(": ");
                        fmt_type(value, buf);
                        if i + 1 < properties.len() {
                            buf.push(',');
                        }
                    }
                }
                buf.push('}');
            }
            _ => {}
        }
    } else if let Some(vty) = ty.get("$ref") {
        let ty = vty.as_str().unwrap().split("/").last().unwrap();
        buf.push_str(ty);
    }
}
