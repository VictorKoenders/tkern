use super::parse::{Field, FieldTy};
use proc_macro2::{Ident, Literal, TokenStream};
use quote::quote;
use std::fmt::Write;

pub fn read_types_and_methods<'a>(fields: &'a [Field]) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut types = Vec::new();
    let mut methods = Vec::new();
    for field in fields.iter().filter(|f| f.readable) {
        let name = &field.name;
        let fields = match &field.ty_info {
            FieldTy::Bool { bit, reset: _ } => {
                let index = bit / 32;
                let offset = bit % 32;
                quote! {
                    pub const fn is_set(&self) -> bool {
                        let mut bytes = (self.0).0;

                        let idx: usize = (bytes.len() - 1 - #index);
                        let offset: usize = #offset;

                        (bytes[idx] & (1 << offset)) > 0
                    }

                    pub const fn is_clear(&self) -> bool {
                        !self.is_set()
                    }
                }
            }
            FieldTy::U8 { range, reset: _ } => {
                let index = range.start / 32;
                let offset = range.start % 32;
                let length = range.end - range.start;
                let mask = format!("0b{:08b}", ((1 << (length + 1)) - 1))
                    .parse::<Literal>()
                    .unwrap();
                quote! {
                    pub const fn get_value(self) -> u8 {
                        let mut bytes = (self.0).0;
                        let idx: usize = (bytes.len() - 1 - #index);
                        let offset: usize = #offset;
                        let mask: u8 = #mask;

                        ((bytes[idx] >> offset) as u8) & mask
                    }
                }
            }
        };

        let type_name = Ident::new(&format!("{}_R", name), name.span());

        types.push(quote! {
            pub struct #type_name<'a>(&'a R);
            impl #type_name<'_> {
                #fields
            }
        });
        let name_to_lower = Ident::new(&name.to_string().to_lowercase(), name.span());
        let docs = &field.docs;
        methods.push(quote! {
            #(#docs)*
            pub const fn #name_to_lower(&self) -> #type_name<'_> {
                #type_name(self)
            }
        });
    }
    (types, methods)
}

pub fn write_default_types_and_methods<'a>(
    fields: &'a [Field],
) -> (TokenStream, Vec<TokenStream>, Vec<TokenStream>) {
    let mut default_impl = String::new();
    let mut types = Vec::new();
    let mut methods = Vec::new();
    let mut default_impl_has_unsafe = false;

    for field in fields.iter().filter(|f| f.writable) {
        let name = &field.name;
        let name_to_lower = Ident::new(&name.to_string().to_lowercase(), name.span());

        let fields = match &field.ty_info {
            FieldTy::Bool { bit, reset } => {
                let index = bit / 32;
                let offset = bit % 32;
                if let Some(reset) = reset {
                    let _ = write!(
                        &mut default_impl,
                        ".{}().{}()",
                        name_to_lower,
                        if *reset { "set" } else { "clear" }
                    );
                }
                quote! {
                    pub const fn set(self) -> W {
                        let mut bytes = (self.0).0;

                        let idx: usize = (bytes.len() - 1 - #index);
                        let offset: usize = #offset;

                        bytes[idx] |= 1 << offset;
                        W(bytes)
                    }

                    pub const fn clear(self) -> W {
                        let mut bytes = (self.0).0;

                        let idx: usize = (bytes.len() - 1 - #index);
                        let offset: usize = #offset;

                        bytes[idx] &= !(1 << offset);
                        W(bytes)
                    }
                }
            }
            FieldTy::U8 { range, reset } => {
                if let Some(reset) = reset {
                    let _ = write!(
                        &mut default_impl,
                        ".{}().set_value({})",
                        name_to_lower, reset
                    );
                    default_impl_has_unsafe = true;
                }
                let index = range.start / 32;
                let offset = range.start % 32;
                let length = range.end - range.start;
                let mask = format!("0b{:08b}", ((1 << (length + 1)) - 1))
                    .parse::<Literal>()
                    .unwrap();
                quote! {
                    pub const unsafe fn set_value(self, value: u8) -> W {
                        let mut bytes = (self.0).0;
                        let idx: usize = (bytes.len() - 1 - #index);
                        let offset: usize = #offset;
                        let mask: u8 = #mask;

                        bytes[idx] |= ((value & mask) as u32) << offset;
                        W(bytes)
                    }
                }
            }
        };

        let type_name = Ident::new(&format!("{}_W", name), name.span());
        types.push(quote! {
            pub struct #type_name(W);
            impl #type_name {
                #fields
            }
        });
        let name_to_lower = Ident::new(&name.to_string().to_lowercase(), name.span());
        let docs = &field.docs;
        methods.push(quote! {
            #(#docs)*
            pub const fn #name_to_lower(self) -> #type_name {
                #type_name(self)
            }
        });
    }
    let mut default_impl: TokenStream = default_impl.parse().unwrap();
    default_impl = quote! { Self(Default::default()) #default_impl };
    if default_impl_has_unsafe {
        default_impl = quote! { unsafe { #default_impl } };
    }
    default_impl = quote! {
        impl Default for W {
            fn default() -> Self {
                #default_impl
            }
        }
    };
    (default_impl, types, methods)
}
