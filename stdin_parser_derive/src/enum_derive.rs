use crate::common;
use quote::quote;

fn default_variant_description(variant: &syn::Variant) -> String {
    variant.ident.to_string()
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
                #ident(#ty::parse_stdin()?)
            }
        }
        syn::Fields::Unit => quote! {#ident},
        syn::Fields::Named(_) => panic!("I didn't expect to see this"),
    }
}

pub(crate) fn derive(ty: &syn::Ident, data: &syn::DataEnum) -> proc_macro2::TokenStream {
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

    let n = variant_constructors.len();

    quote! {
        println!("Choose one of the following:");

        #(#variant_descriptions)*

        loop {
            let index: usize = StdinParser::parse_stdin()?;

            let value = match index {
                #(#variant_constructors, )*
                _ => {
                    println!("provided integer was not in the range 0..{}", #n);
                    continue;
                }
            };

            return Ok(value)
        }
    }
}
