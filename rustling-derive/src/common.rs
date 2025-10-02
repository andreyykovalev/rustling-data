use proc_macro2::TokenStream;
use syn::DeriveInput;

pub struct RepositoryMeta {
    pub entity: TokenStream,
    pub id: TokenStream,
    pub storage_name: String,
}

pub fn parse_repository_meta(ast: &DeriveInput, storage_attr: &str) -> RepositoryMeta {
    let entity_attr = ast
        .attrs
        .iter()
        .find(|a| a.path().is_ident("entity"))
        .expect("Missing #[entity(Type)]");

    let id_attr = ast
        .attrs
        .iter()
        .find(|a| a.path().is_ident("id"))
        .expect("Missing #[id(Type)]");

    let entity: syn::Type = entity_attr.parse_args().unwrap();
    let id: syn::Type = id_attr.parse_args().unwrap();

    let storage_attr = ast.attrs.iter().find(|a| a.path().is_ident(storage_attr));

    let storage_name = if let Some(attr) = storage_attr {
        attr.parse_args::<syn::LitStr>()
            .expect("Expected string literal")
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

    RepositoryMeta {
        entity: quote::quote! { #entity },
        id: quote::quote! { #id },
        storage_name,
    }
}
