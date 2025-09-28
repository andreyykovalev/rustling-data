use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};
use heck::ToSnakeCase;

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // table name = struct name converted to snake_case + "s"
    let table_name = name.to_string().to_snake_case();
    let table_name = format!("{}s", table_name);

    let expanded = quote! {
        impl Entity for #name {
            fn table_name() -> &'static str {
                #table_name
            }
        }
    };

    expanded.into()
}