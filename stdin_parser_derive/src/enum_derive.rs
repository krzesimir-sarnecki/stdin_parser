use crate::common;
use quote::quote;

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

pub(crate) fn derive(ty: &syn::Ident, data: syn::DataEnum) -> proc_macro2::TokenStream {
    let variant_constructors: Vec<_> = data
        .variants
        .iter()
        .enumerate()
        .map(|(index, variant)| {
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
