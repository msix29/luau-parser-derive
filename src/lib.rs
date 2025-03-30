mod range;

use proc_macro::TokenStream;

#[proc_macro_derive(Range)]
#[inline]
pub fn range(input: TokenStream) -> TokenStream {
    range::generate(input)
}
