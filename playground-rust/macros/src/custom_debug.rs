use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

pub fn custom_debug_impl(DeriveInput { ident, data, .. }: DeriveInput) -> TokenStream {
    match data {
        Data::Struct(ds) => {
            let fields = match ds.fields {
                Fields::Named(fields) => fields
                    .named
                    .iter()
                    .map(|f| {
                        let ident = f.ident.clone().unwrap();
                        let field_name = format!("{}", ident.to_string());
                        quote! {
                            .field(#field_name, &self.#ident)
                        }
                    })
                    .collect(),
                _ => quote! {},
            };

            let ds_name = format!("{}", ident.to_string());

            quote! {
                impl std::fmt::Debug for #ident {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.debug_struct(#ds_name)
                         #fields
                         .finish()
                    }
                }
            }
        }
        _ => quote! {},
    }
}
