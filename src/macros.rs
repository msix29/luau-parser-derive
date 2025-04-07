//! Helpful macros for the crate

/// Send an error
macro_rules! error {
    ($message: expr) => {{
        let err = syn::Error::new(
            proc_macro2::Span::call_site(),
            $message
        )
        .to_compile_error();

        quote!(#err).into()
    }};
}
