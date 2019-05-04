extern crate proc_macro;

mod common;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(StdinParser)]
pub fn derive_stdin_parser(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let ty = ast.ident;

    let trait_impl = match ast.data {
        syn::Data::Struct(data) => derive_struct_impl(&ty, data),
        syn::Data::Enum(data) => derive_enum_impl(&ty, data),
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

fn default_field_description(field: &syn::Field) -> String {
    format!("{}", field.ident.as_ref().unwrap().to_string())
}

fn get_field_description(field: &syn::Field) -> String {
    common::get_description(&field.attrs).unwrap_or_else(|| default_field_description(field))
}

fn default_variant_description(variant: &syn::Variant) -> String {
    format!("{}", variant.ident.to_string())
}

fn get_variant_description(variant: &syn::Variant) -> String {
    common::get_description(&variant.attrs).unwrap_or_else(|| default_variant_description(variant))
}

fn get_variant_constructor(variant: &syn::Variant) -> proc_macro2::TokenStream {
    let ident = &variant.ident;

    match &variant.fields {
        syn::Fields::Unnamed(fields) => {
            assert_eq!(
                fields.unnamed.len(),
                1,
                "Currently enum variants can have up to one associated value each"
            );

            // this unwrap is safe as we just checked that there is exactly one field
            let ty = &fields.unnamed.iter().next().as_ref().unwrap().ty;

            quote! {
                #ident(#ty::parse_stdin())
            }
        }
        syn::Fields::Unit => quote! {#ident},
        syn::Fields::Named(_) => panic!("I didn't expect to see this"),
    }
}

fn derive_enum_impl(ty: &syn::Ident, data: syn::DataEnum) -> proc_macro2::TokenStream {
    let variant_constructors: Vec<_> = data
        .variants
        .iter()
        .enumerate()
        .map(|(index, variant)| {
            let ident = &variant.ident;
            let constructor = get_variant_constructor(variant);
            quote! {#index => #ty::#constructor}
        })
        .collect();

    let variant_descriptions: Vec<_> = data
        .variants
        .iter()
        .enumerate()
        .map(|(index, variant)| {
            let description = get_variant_description(variant);
            quote! {
                println!("{}) {}", #index, #description);
            }
        })
        .collect();

    quote! {
        println!("Choose one of the following:");

        #(#variant_descriptions)*

        let index: usize = StdinParser::parse_stdin();
        match index {
            #(#variant_constructors, )*
            _ => panic!("index out of range"),
        }
    }
}

fn derive_struct_impl(ty: &syn::Ident, data: syn::DataStruct) -> proc_macro2::TokenStream {
    let field_names: Vec<_> = data
        .fields
        .iter()
        .map(|field| {
            field
                .ident
                .as_ref()
                .expect("StdinParser only works with names fields")
        })
        .collect();

    let field_descriptions: Vec<_> = data.fields.iter().map(get_field_description).collect();

    let fields_parsers: Vec<_> = field_names
        .iter()
        .zip(field_descriptions.iter())
        .map(|(field_name, field_description)| {
            quote! {
                println!("{}", #field_description);
                let #field_name = StdinParser::parse_stdin();
            }
        })
        .collect();

    quote! {
        #(#fields_parsers)*

        Self{
            #(#field_names, )*
        }
    }
}
