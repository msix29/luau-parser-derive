use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use super::CodeData;

pub fn generate<T: ToTokens + Clone>(names: Vec<T>) -> CodeData<T> {
    assert!(!names.is_empty());

    if names.len() == 1 {
        CodeData {
            first: names[0].clone(),
            middle: Vec::new(),
            last: None,
        }
    } else if names.len() == 2 {
        CodeData {
            first: names[0].clone(),
            middle: Vec::new(),
            last: Some(names[1].clone()),
        }
    } else {
        let first = names[0].clone();
        let middle = names[1..names.len() - 1].to_vec();
        let last = Some(names[names.len() - 1].clone());

        CodeData {
            first,
            middle,
            last,
        }
    }
}

pub fn generate_print<A, B, C>(data: &CodeData<A, B, C>) -> TokenStream
where
    A: ToTokens,
    B: ToTokens,
    C: ToTokens,
{
    let first = &data.first;
    let middle = &data.middle;
    let last = &data.last;

    let last_operation = if let Some(last) = last {
        quote! { start = start.trim_end().to_string() + &#last.print(); }
    } else {
        quote! {}
    };

    quote! {
        let mut start = #first.print();
        #(start = start.trim_end().to_string() + &#middle.print();)*
        #last_operation

        start
    }
}
