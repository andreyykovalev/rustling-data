use crate::common::parse_repository_meta;
use proc_macro::TokenStream;
use quote::quote;
use syn;

pub fn repository_derive(input: TokenStream) -> TokenStream {
    let syntax_tree = syn::parse(input).unwrap();
    implement_repository_trait(&syntax_tree)
}

fn implement_repository_trait(syntax_tree: &syn::DeriveInput) -> TokenStream {
    let name = &syntax_tree.ident;
    let meta = parse_repository_meta(syntax_tree, "table");

    let entity = meta.entity;
    let id = meta.id;
    let storage_name = syn::LitStr::new(&meta.storage_name, proc_macro2::Span::call_site());

    let gene = quote! {
        #[async_trait::async_trait]
        impl Repository<#entity, #id> for #name {
            async fn find_all(&self) -> Result<Vec<#entity>, anyhow::Error> {
                let result = ::rustling_data::PostgresDriver::find_all(
                    &self.pool, // Executor (&PgPool) is passed as the first argument
                    #storage_name
                ).await?;
                Ok(result)
            }
        }
    };
    gene.into()
}
