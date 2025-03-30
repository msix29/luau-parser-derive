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

macro_rules! must_have_one_item {
    () => {
        return error!("Structs passed to `#[Derive(Range)]` must have at least one item.")
    };
}
