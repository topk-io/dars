use syn::{
    Attribute, Expr, Field, Ident, Lit, MetaNameValue, Token, braced,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

#[derive(Debug)]
struct InputField {
    name: String,
    desc: Option<String>,
}

#[derive(Debug)]
struct OutputField {
    name: String,
    desc: Option<String>,
}

#[derive(Debug)]
pub struct Instruction {
    instruction: Option<String>,
}

impl Parse for Instruction {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut instruction = String::new();
        while let Ok(lit) = input.parse::<Lit>() {
            match lit {
                Lit::Str(str) => {
                    for line in str.value().lines() {
                        let line = line.trim();
                        if !line.is_empty() {
                            instruction.push_str(line);
                            instruction.push(' ');
                        }
                    }
                }
                _ => return Err(syn::Error::new(lit.span(), "Expected string literal")),
            }
        }

        Ok(Instruction {
            instruction: (!instruction.is_empty()).then_some(instruction),
        })
    }
}

#[derive(Debug)]
pub struct Signature {
    name: String,
    instruction: Option<String>,
    inputs: Vec<InputField>,
    outputs: Vec<OutputField>,
}

impl Signature {
    pub fn with_instruction(self, instruction: Instruction) -> Self {
        Self {
            instruction: instruction.instruction,
            ..self
        }
    }
}

impl Parse for Signature {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _ = input.parse::<Token![struct]>()?;
        let name: Ident = input.parse()?;
        println!("name: {:?}", name);

        // Extract the content of the struct
        let content;
        braced!(content in input);

        // Parse input/output fields
        let fields = content.parse_terminated(Field::parse_named, Token![,])?;

        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        for field in fields {
            let name = field.ident.expect("Missing field name").to_string();

            if field.attrs.is_empty() {
                panic!("Missing input/output attribute on field {}", name);
            }

            for attr in field.attrs {
                if attr.path().is_ident("input") {
                    inputs.push(InputField {
                        name,
                        desc: parse_desc(&attr)?,
                    });
                } else if attr.path().is_ident("output") {
                    outputs.push(OutputField {
                        name,
                        desc: parse_desc(&attr)?,
                    });
                } else {
                    return Err(syn::Error::new(
                        attr.span(),
                        format!("Unknown attribute on field {name}"),
                    ));
                }
                break;
            }
        }

        Ok(Signature {
            name: name.to_string(),
            instruction: None,
            inputs,
            outputs,
        })
    }
}

fn parse_desc(attr: &Attribute) -> syn::Result<Option<String>> {
    let args = attr.parse_args::<MetaNameValue>()?;
    match args.path.get_ident() {
        Some(ident) => {
            if ident == "desc" {
                match args.value {
                    Expr::Lit(lit) => match lit.lit {
                        Lit::Str(str) => Ok(Some(str.value())),
                        _ => return Err(syn::Error::new(lit.span(), "Expected string literal")),
                    },
                    _ => {
                        return Err(syn::Error::new(
                            args.value.span(),
                            "Expected string literal",
                        ));
                    }
                }
            } else {
                return Err(syn::Error::new(
                    attr.span(),
                    format!("Invalid parameter: {ident}"),
                ));
            }
        }
        None => {
            return Err(syn::Error::new(attr.span(), "Missing attribute name"));
        }
    }
}
