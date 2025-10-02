use crate::common::parse_repository_meta;
use proc_macro::TokenStream;
use quote::quote;
use syn;

pub fn mongo_repository_derive(input: TokenStream) -> TokenStream {
    let syntax_tree = syn::parse(input).unwrap();
    implement_mongo_repository_trait(&syntax_tree)
}

fn implement_mongo_repository_trait(syntax_tree: &syn::DeriveInput) -> TokenStream {
    let name = &syntax_tree.ident;
    let meta = parse_repository_meta(syntax_tree, "collection");

    let entity = meta.entity;
    let id = meta.id;
    let storage_name = syn::LitStr::new(&meta.storage_name, proc_macro2::Span::call_site());

    let generated = quote! {
        #[async_trait::async_trait]
        impl Repository<#entity, #id> for #name {
            async fn find_all(&self) -> Result<Vec<#entity>, anyhow::Error> {
                let mongo_repo = ::rustling_data::MongoDriver::new(self.client.clone(), self.db_name.clone());
                let result = mongo_repo.find_all::<#entity>(#storage_name).await?;
                Ok(result)
    }
        }
    };
    generated.into()
}
