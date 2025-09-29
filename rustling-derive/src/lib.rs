extern crate proc_macro;

use proc_macro::TokenStream;
use rustling_core;
use quote::quote;
use syn::parse::{Parse};


#[proc_macro_derive(HelloWorld)]
pub fn hello_world_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_world(&ast)
}

fn impl_hello_world(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident; // Name of the struct

    let gene = quote! {
        impl HelloWorld for #name {
            fn hello() {
                ::rustling_core::SqlRepository::hello();
            }
        }
    };
    gene.into()
}
