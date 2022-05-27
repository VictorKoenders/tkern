use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod reg_enum;

#[proc_macro_derive(RegEnum)]
pub fn reg_enum_derive(stream: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(stream as DeriveInput);
    match reg_enum::derive(derive) {
        Ok(stream) => stream.into(),
        Err((msg, span)) => syn::Error::new(span, msg).into_compile_error().into(),
    }
}
