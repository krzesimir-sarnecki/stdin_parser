use syn::Attribute;

pub(crate) fn get_description(attrs: &[Attribute]) -> Option<String> {
    let mut description: Vec<String> = Vec::new();

    for attribute in attrs {
        if let syn::AttrStyle::Outer = attribute.style {
            let meta = attribute.parse_meta().unwrap();
            if let syn::Meta::NameValue(value) = meta {
                if value.ident.to_string() == "doc" {
                    if let syn::Lit::Str(lit) = value.lit {
                        description.push(lit.value().trim().to_string());
                    }
                }
            }
        }
    }

    if description.is_empty() {
        None
    } else {
        Some(description.join("\n"))
    }
}
