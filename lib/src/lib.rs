use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn function_to_string(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse input function
    let func: ItemFn = parse_macro_input!(item as ItemFn);

    // Get function body
    let func_body = format!("{}", func.to_token_stream());

    // Define a new function with same signature as the input function
    let func_ident = func.sig.ident;
    let func_generics = func.sig.generics;
    let func_inputs = func.sig.inputs;

    let output = quote! {
        pub fn #func_ident #func_generics(#func_inputs) -> &'static str {
            #func_body
        }
    };
    output.into()
}
