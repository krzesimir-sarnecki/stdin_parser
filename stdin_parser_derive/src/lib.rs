extern crate proc_macro;

mod common;
mod enum_derive;
mod struct_derive;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(StdinParser)]
pub fn derive_stdin_parser(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let ty = ast.ident;

    let trait_impl = match ast.data {
        syn::Data::Struct(data) => struct_derive::derive(&ty, data),
        syn::Data::Enum(data) => enum_derive::derive(&ty, data),
        _ => panic!("StdinParser only works with structs and enums"),
    };

    let gen = quote! {
        impl StdinParser for #ty {
            fn parse_stdin() -> Self{
                #trait_impl
            }
        }
    };

    gen.into()
}
