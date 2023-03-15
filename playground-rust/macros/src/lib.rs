use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Foo)]
pub fn foo_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_foo(&ast)
}
