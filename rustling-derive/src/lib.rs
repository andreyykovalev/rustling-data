extern crate proc_macro;

use proc_macro::TokenStream;
use rustling_core;
use quote::quote;


#[proc_macro_derive(HelloWorld)]
pub fn hello_world_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let syntax_tree = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_world(&syntax_tree)
}

#[proc_macro_derive(Repository, attributes(entity, id))]
pub fn repository_derive(input: TokenStream) -> TokenStream {
    let syntax_tree = syn::parse(input).unwrap();

    impl_repository(&syntax_tree)
}

fn impl_hello_world(syntax_tree: &syn::DeriveInput) -> TokenStream {
    let name = &syntax_tree.ident; // Name of the struct

    let gene = quote! {
        impl HelloWorld for #name {
            fn hello() {
                ::rustling_core::SqlRepository::hello();
            }
        }
    };
    gene.into()
}

fn impl_repository(syntax_tree: &syn::DeriveInput) -> TokenStream {
    let name = &syntax_tree.ident; // Name of the struct

    let (entity, id) = get_entity_and_id(syntax_tree);

    let gene = quote! {
        #[async_trait::async_trait]
        impl Repository<#entity, #id> for #name {
            async fn find_all(&self) -> Result<Vec<#entity>, anyhow::Error> {
                // Instantiate SqlRepository
                let sql_repo = ::rustling_core::SqlRepository {};

                // Call the async method
                let result: Vec<#entity> = sql_repo.find_all().await?;
                Ok(result)
            }
        }
    };
    gene.into()
}

fn get_entity_and_id(ast: &syn::DeriveInput) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let entity_attr = ast.attrs.iter()
        .find(|attr| attr.path().is_ident("entity"))
        .expect("Missing #[entity(Type)]");

    let id_attr = ast.attrs.iter()
        .find(|attr| attr.path().is_ident("id"))
        .expect("Missing #[id(Type)]");

    let entity: syn::Type = entity_attr.parse_args().unwrap();
    let id: syn::Type = id_attr.parse_args().unwrap();

    (quote! { #entity }, quote! { #id })
}
