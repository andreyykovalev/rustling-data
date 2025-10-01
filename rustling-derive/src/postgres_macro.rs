// src/repository_macro.rs

use proc_macro::TokenStream;
use quote::quote;

pub fn repository_derive(input: TokenStream) -> TokenStream {
    let syntax_tree = syn::parse(input).unwrap();
    implement_repository_trait(&syntax_tree)
}

fn implement_repository_trait(syntax_tree: &syn::DeriveInput) -> TokenStream {
    let name = &syntax_tree.ident; // Name of the struct

    let (entity, id, table_name) = get_entity_and_id(syntax_tree);

    let gene = quote! {
        #[async_trait::async_trait]
        impl Repository<#entity, #id> for #name {
            async fn find_all(&self) -> Result<Vec<#entity>, anyhow::Error> {
                let sql_repo = ::rustling_data::PostgresDriver::new(self.pool.clone());
                let result = sql_repo.find_all::<#entity>(#table_name).await?;
                Ok(result)
            }
        }
    };
    gene.into()
}

fn get_entity_and_id(
    ast: &syn::DeriveInput,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream, String) {
    let entity_attr = ast
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("entity"))
        .expect("Missing #[entity(Type)]");

    let id_attr = ast
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("id"))
        .expect("Missing #[id(Type)]");

    let entity: syn::Type = entity_attr.parse_args().unwrap();
    let id: syn::Type = id_attr.parse_args().unwrap();

    let table_attr = ast.attrs.iter().find(|attr| attr.path().is_ident("table"));

    let table_name = if let Some(attr) = table_attr {
        attr.parse_args::<syn::LitStr>()
            .expect("Expected string literal in #[table]")
            .value()
    } else {
        let ident = match entity {
            syn::Type::Path(ref p) if p.qself.is_none() => {
                p.path.segments.last().unwrap().ident.to_string()
            }
            _ => panic!("Unsupported entity type"),
        };
        ident.to_lowercase() + "s"
    };

    (quote! { #entity }, quote! { #id }, table_name)
}
