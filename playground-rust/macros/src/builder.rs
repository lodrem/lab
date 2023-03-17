use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    AngleBracketedGenericArguments, Data, DataStruct, DeriveInput, Fields, GenericArgument, Path,
    PathArguments, PathSegment, Type, TypePath,
};

pub(crate) fn builder_impl(DeriveInput { ident, data, .. }: DeriveInput) -> TokenStream {
    match data {
        Data::Struct(ds) => {
            let builder_ident = Ident::new(&format!("{}Builder", ident.to_string()), ident.span());
            let builder_fields = to_builder_fields(&ds);
            let builder_field_setter = to_builder_field_setter(&ds);
            let builder_build = to_builder_build(&ident, &ds);

            quote! {
                impl #ident {
                    pub fn builder() -> #builder_ident {
                        #builder_ident::default()
                    }
                }

                #[derive(Default)]
                pub struct #builder_ident {
                    #builder_fields
                }

                impl #builder_ident {
                    #builder_field_setter

                    #builder_build
                }
            }
        }
        _ => quote! {},
    }
}

fn to_builder_fields(ds: &DataStruct) -> TokenStream {
    match &ds.fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|f| {
                let ident = f.ident.as_ref().unwrap();
                let typ = try_optional(&f.ty).unwrap_or(f.ty.clone());
                quote! {
                    #ident: std::option::Option<#typ>,
                }
            })
            .collect(),
        _ => quote! {},
    }
}

fn to_builder_field_setter(ds: &DataStruct) -> TokenStream {
    match &ds.fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|f| {
                let ident = f.ident.as_ref().unwrap();
                let fn_ident = Ident::new(&ident.to_string(), ident.span());
                let typ = try_optional(&f.ty).unwrap_or(f.ty.clone());
                quote! {
                    pub fn #fn_ident(&mut self, value: #typ) -> &mut Self {
                        self.#ident = Some(value);
                        self
                    }
                }
            })
            .collect(),
        _ => quote! {},
    }
}

fn to_builder_build(ident: &Ident, ds: &DataStruct) -> TokenStream {
    let fields = match &ds.fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|f| {
                let ident = f.ident.as_ref().unwrap();
                let field_name = ident.to_string();
                if try_optional(&f.ty).is_some() {
                    quote! {
                        #ident: self.#ident.take(),
                    }
                } else {
                    quote! {
                        #ident: self.#ident.take().ok_or_else(|| anyhow::anyhow!("Missing attribute: {}", #field_name))?,
                    }
                }
            })
            .collect(),
        _ => quote! {},
    };

    quote! {
        pub fn build(&mut self) -> std::result::Result<#ident, Box<dyn std::error::Error>> {
            Ok(#ident {
                #fields
            })
        }
    }
}

fn try_optional(ty: &Type) -> Option<Type> {
    let segments = match ty {
        Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) if segments.len() == 1 => segments.clone(),
        _ => return None,
    };
    let args = match &segments[0] {
        PathSegment {
            ident,
            arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }),
        } if ident == "Option" && args.len() == 1 => args,
        _ => return None,
    };
    let ty = match &args[0] {
        GenericArgument::Type(ty) => ty,
        _ => return None,
    };
    Some(ty.clone())
}
