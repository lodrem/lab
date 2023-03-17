mod builder;
mod custom_debug;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro]
pub fn make_answer(item: TokenStream) -> TokenStream {
    let answer = 42;
    let expanded = quote! {
        pub fn answer() -> i32 {
            #answer
        }
    };
    println!("make_answer -> {}", item.to_string());
    TokenStream::from(expanded)
}

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(item: TokenStream) -> TokenStream {
    let answer = 42;
    let expanded = quote! {
        pub fn answer_fn() -> i32 {
            #answer
        }
    };
    println!("make_answer_fn -> {}", item.to_string());
    TokenStream::from(expanded)
}

#[proc_macro_derive(HelperAttr, attributes(helper))]
pub fn derive_helper_attr(item: TokenStream) -> TokenStream {
    println!("make_helper_attr -> {}", item.to_string());
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = builder::builder_impl(input);
    TokenStream::from(output)
}

#[proc_macro_derive(CustomDebug, attributes(format))]
pub fn derive_custom_debug(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = custom_debug::custom_debug_impl(input);
    TokenStream::from(output)
}
