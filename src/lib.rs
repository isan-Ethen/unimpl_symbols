use proc_macro::TokenStream;
use quote::quote;
use std::fs::OpenOptions;
use std::io::Write;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, ItemFn,
};

struct UnimplementedInput {
    functions: Vec<ItemFn>,
}

impl Parse for UnimplementedInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut functions = Vec::new();
        while !input.is_empty() {
            functions.push(input.parse()?);
        }
        Ok(UnimplementedInput { functions })
    }
}

#[proc_macro]
pub fn unimplemented_functions(input: TokenStream) -> TokenStream {
    let UnimplementedInput { functions } = parse_macro_input!(input as UnimplementedInput);

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR must be set");
    let dest_path = std::path::Path::new(&out_dir).join("unimplemented_symbols.txt");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&dest_path)
        .expect("Failed to open unimplemented_symbols.txt");

    let generated_functions = functions.iter().map(|func| {
        let sig = &func.sig;
        let vis = &func.vis;
        let func_name = &sig.ident;

        writeln!(file, "{}", func_name).expect("Failed to write to file");

        quote! {
            #vis #sig {
                unimplemented!();
            }
        }
    });

    let result = quote! {
        #(#generated_functions)*
    };

    result.into()
}
