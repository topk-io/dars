use syn::{
    Lit,
    parse::{Parse, ParseStream},
};

#[derive(Debug)]
pub struct Args {
    args: Option<String>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = String::new();
        while let Ok(lit) = input.parse::<Lit>() {
            match lit {
                Lit::Str(str) => {
                    for line in str.value().lines() {
                        let line = line.trim();
                        if !line.is_empty() {
                            args.push_str(line);
                            args.push('\n');
                        }
                    }
                }
                _ => return Err(syn::Error::new(lit.span(), "Expected string literal")),
            }
        }

        Ok(Args {
            args: (!args.is_empty()).then_some(args),
        })
    }
}

impl Into<Option<String>> for Args {
    fn into(self) -> Option<String> {
        self.args
    }
}
