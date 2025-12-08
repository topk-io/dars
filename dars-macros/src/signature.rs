use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::{
    Attribute, Expr, Field, Ident, Lit, MetaNameValue, Token, Type, braced,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

struct InputField {
    name: String,
    ty: Type,
    desc: Option<String>,
}

struct OutputField {
    name: String,
    ty: Type,
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

pub struct Signature {
    name: String,
    instruction: Option<String>,
    inputs: Vec<InputField>,
    outputs: Vec<OutputField>,
}

impl Signature {
    pub(crate) fn with_instruction(self, instruction: Instruction) -> Self {
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
                        ty: field.ty,
                        desc: parse_desc(&attr)?,
                    });
                    break;
                }
                if attr.path().is_ident("output") {
                    outputs.push(OutputField {
                        name,
                        ty: field.ty,
                        desc: parse_desc(&attr)?,
                    });
                    break;
                }
                return Err(syn::Error::new(
                    attr.span(),
                    format!("Unknown attribute on field {name}"),
                ));
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

impl ToTokens for Signature {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let instruction = self.instruction.as_ref();
        println!("instruction: {:?}", instruction);
        let name = format_ident!("{}", self.name);

        let input_struct = format_ident!("{}Input", self.name);
        let inputs = self.inputs.iter().map(|input| {
            let name = Ident::new(&input.name, Span::call_site());
            let ty = input.ty.clone();
            quote! {
                pub #name: #ty
            }
        });

        let output_struct = format_ident!("{}Output", self.name);
        let outputs = self.outputs.iter().map(|output| {
            let name = Ident::new(&output.name, Span::call_site());
            let ty = output.ty.clone();
            quote! {
                pub #name: #ty
            }
        });

        let expanded = quote! {
            // Input struct
            #[derive(Debug, dars::serde::Serialize, dars::schemars::JsonSchema)]
            struct #input_struct {
                #(#inputs)*
            }

            // Output struct
            #[derive(Debug, dars::serde::Deserialize, dars::schemars::JsonSchema)]
            struct #output_struct {
                #(#outputs)*
            }

            // Base signature struct
            #[derive(Debug)]
            struct #name {
                instruction: Option<String>,
            }

            impl #name {
                pub fn new() -> Self {
                    Self {
                        instruction: Some("foo".into())
                    }
                }
            }

            impl dars::Signature for #name {
                type Input = #input_struct;
                type Output = #output_struct;
            }
        };
        tokens.extend(expanded);
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
