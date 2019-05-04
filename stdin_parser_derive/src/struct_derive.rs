use crate::common;
use quote::quote;

fn default_field_description(field: &syn::Field) -> String {
    format!("{}", field.ident.as_ref().unwrap().to_string())
}

fn get_field_description(field: &syn::Field) -> String {
    common::get_description(&field.attrs).unwrap_or_else(|| default_field_description(field))
}

pub(crate) fn derive(_ty: &syn::Ident, data: syn::DataStruct) -> proc_macro2::TokenStream {
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
