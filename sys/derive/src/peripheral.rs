use proc_macro2::{Ident, Literal, Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Type};

pub fn derive(input: DeriveInput) -> Result<TokenStream, (String, Span)> {
    let StructAttributes { offset, size } = get_struct_attributes(&input)?;
    let fields = get_struct_fields(&input)?;
    let ident = &input.ident;
    let module_name = Ident::new(&ident.to_string().to_lowercase(), ident.span());
    let r_fields = fields.iter().filter_map(|f| {
        if !f.readable {
            None
        } else {
            let name = &f.name;
            let ty = &f.ty;
            Some(quote! { pub #name: #ty, })
        }
    });
    Ok(quote! {
        pub mod #module_name {
            pub struct R {
                #(#r_fields, )*
            }
            pub struct W;
        }

        impl super::Peripheral for #ident {
            type R = #module_name::R;
            type W = #module_name::W;

            unsafe fn read(base_address: core::ptr::NonNull<()>) -> Self::R {
                use core::num::NonZeroUsize;
                let bytes = unsafe { core::ptr::read_volatile(
                    base_address.map_addr(|addr| {
                        NonZeroUsize::new(addr.get() + #offset).unwrap()
                    }).cast().as_ptr()
                ) };
                #module_name::R::from_bytes(bytes)
            }

            unsafe fn write(base_address: core::ptr::NonNull<()>, value: Self::W) {
                use core::num::NonZeroUsize;
                let bytes = value.to_bytes();
                unsafe {
                    core::ptr::write_volatile(
                        base_address.map_addr(|addr| {
                            NonZeroUsize::new(addr.get() + #offset).unwrap()
                        }).cast().as_ptr(),
                        bytes
                    );
                }
            }
        }
    })
}

macro_rules! some_or_continue {
    ($e:expr) => {
        if let Some(val) = $e {
            val
        } else {
            continue;
        }
    };
}
fn get_struct_attributes(input: &DeriveInput) -> Result<StructAttributes, (String, Span)> {
    for attr in &input.attrs {
        let mut segments = attr.path.segments.iter();
        let path = some_or_continue!(segments.next());
        if path.ident != "peripheral" {
            continue;
        }
        let group = match attr.tokens.clone().into_iter().next() {
            Some(TokenTree::Group(group)) => group,
            _ => return Err(("Missing properties".to_string(), path.ident.span())),
        };
        let mut offset = None;
        let mut tokens = group.stream().into_iter();
        while let Some(token) = tokens.next() {
            match token {
                TokenTree::Punct(p) if p.as_char() == ',' => continue,
                TokenTree::Ident(i) if i == "address" => {
                    match tokens.next() {
                        Some(TokenTree::Punct(p)) if p.as_char() == '=' => {}
                        _ => return Err(("Expected `=`".to_string(), i.span())),
                    }
                    match tokens.next() {
                        Some(TokenTree::Literal(lit)) => offset = Some(lit),
                        _ => return Err(("Expected `address = 0x...`".to_string(), i.span())),
                    }
                }
                x => panic!("Unexpected token {:?}", x),
            }
        }

        let offset = offset.ok_or(("Missing property `offset`".to_string(), path.ident.span()))?;
        return Ok(StructAttributes { offset, size: 4 });
    }
    Err((
        "Missing attribute `peripheral`".to_string(),
        input.ident.span(),
    ))
}

struct StructAttributes {
    size: usize,
    offset: Literal,
}

fn get_struct_fields(input: &DeriveInput) -> Result<Vec<Field>, (String, Span)> {
    let fields = match &input.data {
        Data::Struct(str) => &str.fields,
        _ => return Err(("Only structs are supported".to_string(), input.ident.span())),
    };
    let mut result = Vec::new();
    for field in fields {
        let name = match field.ident.as_ref() {
            Some(ident) => ident.clone(),
            None => {
                return Err((
                    "Tuple structs not supported".to_string(),
                    input.ident.span(),
                ))
            }
        };
        let field_attr = field
            .attrs
            .iter()
            .filter_map(|a| {
                let mut segments = a.path.segments.iter();
                let path = segments.next()?;
                if path.ident == "field" {
                    Some(a)
                } else {
                    None
                }
            })
            .next()
            .ok_or(("Missing property `field`".to_string(), name.span()))?;

        let group = match field_attr.tokens.clone().into_iter().next() {
            Some(TokenTree::Group(group)) => group,
            _ => return Err(("Invalid field".to_string(), name.span())),
        };
        let mut readable = false;
        let mut writable = false;
        let mut reset = None;
        let mut bit = None;

        let mut tokens = group.stream().into_iter();
        while let Some(token) = tokens.next() {
            match token {
                TokenTree::Punct(p) if p.as_char() == ',' => continue,
                TokenTree::Ident(i) if i == "readable" => readable = true,
                TokenTree::Ident(i) if i == "writable" => writable = true,
                TokenTree::Ident(i) if i == "reset" => {
                    match tokens.next() {
                        Some(TokenTree::Punct(p)) if p.as_char() == '=' => {}
                        _ => return Err(("Expected `=`".to_string(), i.span())),
                    }
                    match tokens.next() {
                        Some(tree) => reset = Some(tree),
                        None => return Err(("Expected `reset = <value>`".to_string(), i.span())),
                    }
                }
                TokenTree::Ident(i) if i == "bit" => {
                    match tokens.next() {
                        Some(TokenTree::Punct(p)) if p.as_char() == '=' => {}
                        _ => return Err(("Expected `=`".to_string(), i.span())),
                    }
                    match tokens.next() {
                        Some(TokenTree::Literal(lit)) => bit = Some(lit),
                        _ => return Err(("Expected `bit = <value>`".to_string(), i.span())),
                    }
                }
                x => panic!("Unexpected token {:?}", x),
            }
        }

        let ty = &field.ty;
        let ty_info = ty.to_token_stream().to_string();
        let ty_info = match ty_info.as_str() {
            "bool" => match bit {
                Some(bit) => FieldTy::Bool { bit },
                None => return Err(("Missing `bit = <offset>` field".to_string(), name.span())),
            },
            _ => {
                return Err((
                    format!("Unknown type {:?}, only `bool` is supported", field.ty),
                    name.span(),
                ))
            }
        };

        result.push(Field {
            name,
            readable,
            writable,
            reset,
            ty,
            ty_info,
        });
    }
    Ok(result)
}

struct Field<'a> {
    name: Ident,
    readable: bool,
    writable: bool,
    reset: Option<TokenTree>,

    ty: &'a Type,
    ty_info: FieldTy,
}

enum FieldTy {
    Bool { bit: Literal },
}
