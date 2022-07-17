use super::parse::{Field, FieldTy};
use proc_macro2::{Ident, Literal, TokenStream};
use quote::quote;
use std::fmt::Write;

pub fn read_types_and_methods(fields: &[Field]) -> (Vec<TokenStream>, Vec<TokenStream>) {
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
            FieldTy::U16 { range, reset: _ } => {
                let index = range.start / 32;
                let offset = range.start % 32;
                let length = range.end - range.start;
                let mask = format!("0b{:016b}", ((1 << (length + 1)) - 1))
                    .parse::<Literal>()
                    .unwrap();
                quote! {
                    pub const fn get_value(self) -> u16 {
                        let mut bytes = (self.0).0;
                        let idx: usize = (bytes.len() - 1 - #index);
                        let offset: usize = #offset;
                        let mask: u16 = #mask;

                        ((bytes[idx] >> offset) as u16) & mask
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

pub fn write_default_types_and_methods(
    inner_ty: &TokenStream,
    fields: &[Field],
) -> (TokenStream, Vec<TokenStream>, Vec<TokenStream>) {
    let (types, methods) = generate_write_types_amethods(fields);
    let mut default_impl: TokenStream = generate_default_impl(fields).parse().unwrap();

    default_impl = quote! {
        impl Default for W {
            fn default() -> Self {
                let mut val: #inner_ty = Default::default();
                #default_impl
                Self(val)
            }
        }
    };
    (default_impl, types, methods)
}

fn generate_write_types_amethods(fields: &[Field]) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut types = Vec::new();
    let mut methods = Vec::new();
    for field in fields.iter().filter(|f| f.writable) {
        let name = &field.name;

        let fields = match &field.ty_info {
            FieldTy::Bool { bit, reset: _ } => {
                let index = bit / 32;
                let offset = bit % 32;
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
            FieldTy::U8 { range, reset: _ } => {
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
            FieldTy::U16 { range, reset: _ } => {
                let index = range.start / 32;
                let offset = range.start % 32;
                let length = range.end - range.start;
                let mask = format!("0b{:016b}", ((1 << (length + 1)) - 1))
                    .parse::<Literal>()
                    .unwrap();
                quote! {
                    pub const unsafe fn set_value(self, value: u16) -> W {
                        let mut bytes = (self.0).0;
                        let idx: usize = (bytes.len() - 1 - #index);
                        let offset: usize = #offset;
                        let mask: u16 = #mask;

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
    (types, methods)
}

fn generate_default_impl(fields: &[Field]) -> String {
    let mut result = String::new();
    for field in fields {
        let name = &field.name;

        match &field.ty_info {
            FieldTy::Bool { bit, reset } => {
                if let Some(true) = reset {
                    let index = bit / 32;
                    let offset = bit % 32;
                    let mask = 1 << offset;
                    let _ = writeln!(
                        &mut result,
                        "val[{}] |= 0b{:032b}; // {} = {}",
                        index, mask, name, offset,
                    );
                }
            }
            FieldTy::U8 { range, reset } => {
                if let Some(reset) = reset {
                    let index = range.start / 32;
                    let offset = range.start % 32;
                    let _ = write!(
                        &mut result,
                        "val[{}] |= 0b{:032b}; // {} = {}",
                        index,
                        reset << offset,
                        name,
                        reset
                    );
                }
            }
            FieldTy::U16 { range, reset } => {
                if let Some(reset) = reset {
                    let index = range.start / 32;
                    let offset = range.start % 32;
                    let _ = write!(
                        &mut result,
                        "val[{}] |= 0b{:032b}; // {} = {}",
                        index,
                        reset << offset,
                        name,
                        reset
                    );
                }
            }
        };
    }
    result
}
