use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod peripheral;

#[proc_macro_derive(Peripheral, attributes(peripheral, field))]
pub fn peripheral_derive(stream: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(stream as DeriveInput);
    match peripheral::derive(derive) {
        Ok(stream) => stream.into(),
        Err((msg, span)) => syn::Error::new(span, msg).into_compile_error().into(),
    }
}
