use proc_macro2::Ident;
use syn::{Expr, Field, Lit, Meta};

pub fn get_fallback(field: &Field, name: &Ident) -> (bool, Option<Ident>) {
    let mut fallback_field = None;
    let mut found_attribute = false;

    for attr in &field.attrs {
        if attr.path().is_ident("range_or") {
            found_attribute = true;

            if let Meta::NameValue(meta) = &attr.meta {
                if let Expr::Lit(literal) = &meta.value {
                    if let Lit::Str(lit_str) = &literal.lit {
                        fallback_field = Some(syn::Ident::new(&lit_str.value(), name.span()));
                    }
                }
            }
        }
    }

    (found_attribute, fallback_field)
}
