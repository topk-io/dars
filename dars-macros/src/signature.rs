use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::{
    Field, Ident, Lit, LitStr, Token, Type, Visibility, braced,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

use crate::util::parse_desc;

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
    vis: Visibility,
    name: Ident,
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
        let vis = input.parse::<Visibility>()?;
        let _ = input.parse::<Token![struct]>()?;
        let name: Ident = input.parse()?;

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
            vis,
            name,
            instruction: None,
            inputs,
            outputs,
        })
    }
}

impl ToTokens for Signature {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let instruction = self
            .instruction
            .as_ref()
            .map(|s| s.to_string())
            .unwrap_or_default();

        let name = &self.name;
        let vis = &self.vis;

        // Input fields
        let input_struct = format_ident!("{}Input", self.name);
        let inputs = self.inputs.iter().map(|input| {
            let name = Ident::new(&input.name, Span::call_site());
            let ty = input.ty.clone();
            match &input.desc {
                Some(desc) => {
                    let desc = LitStr::new(desc, Span::call_site());
                    quote! {
                        #[field(desc = #desc)]
                        pub #name: #ty
                    }
                }
                None => {
                    quote! {
                        pub #name: #ty
                    }
                }
            }
        });

        // Output fields
        let output_struct = format_ident!("{}Output", self.name);
        let outputs = self.outputs.iter().map(|output| {
            let name = Ident::new(&output.name, Span::call_site());
            let ty = output.ty.clone();
            match &output.desc {
                Some(desc) => {
                    let desc = LitStr::new(desc, Span::call_site());
                    quote! {
                        #[field(desc = #desc)]
                        pub #name: #ty
                    }
                }
                None => {
                    quote! {
                        pub #name: #ty
                    }
                }
            }
        });

        let expanded = quote! {
            // Input model struct
            #[Model]
            #vis struct #input_struct {
                #(#inputs,)*
            }

            // Output model struct
            #[Model]
            #vis struct #output_struct {
                #(#outputs,)*
            }

            // Base signature struct
            #[derive(Debug)]
            #vis struct #name {
                instruction: String,
                input_schema: dars::schemars::Schema,
                output_schema: dars::schemars::Schema,
            }

            impl #name {
                #vis fn new() -> Self {
                    Self {
                        instruction: #instruction.into(),
                        input_schema: dars::schemars::schema_for!(#input_struct),
                        output_schema: dars::schemars::schema_for!(#output_struct),
                    }
                }
            }

            impl dars::Signature for #name {
                type Input = #input_struct;
                type Output = #output_struct;

                #[inline(always)]
                fn instruction(&self) -> &str {
                    &self.instruction
                }

                #[inline]
                fn input_fields(&self) -> &[dars::Field] {
                    <#input_struct as dars::Model>::fields()
                }

                #[inline(always)]
                fn input_schema(&self) -> &dars::schemars::Schema {
                    &self.input_schema
                }

                #[inline]
                fn output_fields(&self) -> &[dars::Field] {
                    <#output_struct as dars::Model>::fields()
                }

                #[inline(always)]
                fn output_schema(&self) -> &dars::schemars::Schema {
                    &self.output_schema
                }
            }
        };
        tokens.extend(expanded);
    }
}
