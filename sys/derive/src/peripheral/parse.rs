use std::ops::Range;

use proc_macro2::{Ident, Literal, Span, TokenTree};
use quote::ToTokens;
use syn::{Attribute, Data, DeriveInput, Type};

pub fn get_struct_attributes(input: &DeriveInput) -> Result<StructAttributes, (String, Span)> {
    for attr in &input.attrs {
        let mut segments = attr.path.segments.iter();
        let path = match segments.next() {
            Some(path) if path.ident == "peripheral" => path,
            _ => continue,
        };
        let group = match attr.tokens.clone().into_iter().next() {
            Some(TokenTree::Group(group)) => group,
            _ => return Err(("Missing properties".to_string(), path.ident.span())),
        };
        let mut offset = None;
        let mut tokens = group.stream().into_iter();
        let mut inner = "[u32; 1]".to_string();
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
                TokenTree::Ident(i) if i == "bytes" => {
                    match tokens.next() {
                        Some(TokenTree::Punct(p)) if p.as_char() == '=' => {}
                        _ => return Err(("Expected `=`".to_string(), i.span())),
                    }
                    match tokens.next() {
                        Some(TokenTree::Literal(lit)) => {
                            let val = lit.to_string();
                            if let Ok(num) = val.parse::<u32>() {
                                let size = (num + 3) % 4;
                                inner = format!("[u32; {}]", size);
                            } else {
                                return Err((
                                    "Expected `bytes = <4, 8, 12, ...>`".to_string(),
                                    i.span(),
                                ));
                            }
                        }
                        _ => return Err(("Expected `address = 0x...`".to_string(), i.span())),
                    }
                }
                x => panic!("Unexpected struct attribute {}", x),
            }
        }

        let offset = offset.ok_or(("Missing property `offset`".to_string(), path.ident.span()))?;
        return Ok(StructAttributes { offset, inner });
    }
    Err((
        "Missing attribute `peripheral`".to_string(),
        input.ident.span(),
    ))
}

pub struct StructAttributes {
    pub offset: Literal,
    pub inner: String,
}

pub fn get_struct_fields(input: &DeriveInput) -> Result<Vec<Field>, (String, Span)> {
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
        let mut field_attr = None;
        let mut docs = Vec::new();
        for a in &field.attrs {
            match a.path.segments.iter().next() {
                Some(path) if path.ident == "field" => field_attr = Some(a),
                Some(path) if path.ident == "doc" => docs.push(a),
                _ => {}
            }
        }
        let field_attr = field_attr.ok_or(("Missing property `field`".to_string(), name.span()))?;

        let group = match field_attr.tokens.clone().into_iter().next() {
            Some(TokenTree::Group(group)) => group,
            _ => return Err(("Invalid field".to_string(), name.span())),
        };
        let mut readable = false;
        let mut writable = false;
        let mut reset = None;
        let mut bit = None;
        let mut bits = None;
        let mut unsafe_enum = None;
        let mut try_enum = None;

        let mut tokens = group.stream().into_iter();
        while let Some(token) = tokens.next() {
            match token {
                TokenTree::Punct(p) if p.as_char() == ',' => continue,
                TokenTree::Ident(i) if i == "readable" || i == "r" => readable = true,
                TokenTree::Ident(i) if i == "writable" || i == "w" => writable = true,
                TokenTree::Ident(i) if i == "rw" => {
                    readable = true;
                    writable = true;
                }
                TokenTree::Ident(i) if i == "try_enum" => match tokens.next() {
                    Some(TokenTree::Group(group)) => match group.stream().into_iter().next() {
                        Some(TokenTree::Ident(i)) => try_enum = Some(i),
                        _ => return Err(("Expected `try_enum(u8)`".to_string(), group.span())),
                    },
                    _ => return Err(("Expected `try_enum(u8)`".to_string(), i.span())),
                },
                TokenTree::Ident(i) if i == "unsafe_enum" => match tokens.next() {
                    Some(TokenTree::Group(group)) => match group.stream().into_iter().next() {
                        Some(TokenTree::Ident(i)) => unsafe_enum = Some(i),
                        _ => return Err(("Expected `enum(u8)`".to_string(), group.span())),
                    },
                    _ => return Err(("Expected `enum(u8)`".to_string(), i.span())),
                },
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
                TokenTree::Ident(i) if i == "bits" => {
                    match tokens.next() {
                        Some(TokenTree::Punct(p)) if p.as_char() == '=' => {}
                        _ => return Err(("Expected `=`".to_string(), i.span())),
                    }
                    let mut new_bits = Vec::new();
                    loop {
                        match tokens.next() {
                            Some(TokenTree::Punct(p)) if p.as_char() == ',' => break,
                            None => break,
                            Some(t) => new_bits.push(t),
                        }
                    }
                    bits = Some(new_bits);
                }
                x => {
                    return Err((format!("Unexpected field attribute {}", x), x.span()));
                }
            }
        }

        let ty = &field.ty;
        let ty_info = ty.to_token_stream().to_string();
        let ty_info = match ty_info.as_str() {
            "bool" => {
                let reset = if let Some(reset) = reset {
                    let str = reset.to_string();
                    Some(match str.as_str() {
                        "true" | "1" => true,
                        "false" | "0" => false,
                        x => {
                            return Err((format!("Invalid reset value for bool, expected `true`, `false`, `1`, or `0`, got {:?}", x), reset.span()));
                        }
                    })
                } else {
                    None
                };
                match bit {
                    Some(bit) => {
                        if let Ok(bit) = bit.to_string().parse() {
                            FieldTy::Bool { bit, reset }
                        } else {
                            return Err((format!("Expected integer, got {:?}", bit), bit.span()));
                        }
                    }
                    None => {
                        return Err(("Missing `bit = <offset>` field".to_string(), name.span()))
                    }
                }
            }
            "u8" => {
                let reset = if let Some(reset) = reset {
                    let str = reset.to_string();
                    let reset = if let Some(base_2) = str.strip_prefix("0b") {
                        u8::from_str_radix(base_2, 2)
                    } else if let Some(base_16) = str.strip_prefix("0x") {
                        u8::from_str_radix(base_16, 16)
                    } else {
                        str.parse()
                    }
                    .map_err(|_| ("Invalid reset value for u8".to_string(), reset.span()))?;

                    Some(reset)
                } else {
                    None
                };
                let range = parse_range::<u8>(bits, &name)?;
                FieldTy::U8 { range, reset }
            }
            "u16" => {
                let reset = if let Some(reset) = reset {
                    let str = reset.to_string();
                    let reset = if let Some(base_2) = str.strip_prefix("0b") {
                        u16::from_str_radix(base_2, 2)
                    } else if let Some(base_16) = str.strip_prefix("0x") {
                        u16::from_str_radix(base_16, 16)
                    } else {
                        str.parse()
                    }
                    .map_err(|_| ("Invalid reset value for u16".to_string(), reset.span()))?;

                    Some(reset)
                } else {
                    None
                };
                let range = parse_range::<u16>(bits, &name)?;
                FieldTy::U16 { range, reset }
            }
            _ => {
                if let Some(primitive_ty) = unsafe_enum {
                    let range = match primitive_ty.to_string().as_str() {
                        "u8" => parse_range::<u8>(bits, &name)?,
                        "u16" => parse_range::<u16>(bits, &name)?,
                        _ => {
                            return Err((
                                "Only u8/u16 enums are supported at the moment".to_string(),
                                primitive_ty.span(),
                            ))
                        }
                    };
                    FieldTy::UnsafeEnum {
                        range,
                        primitive_ty,
                    }
                } else if let Some(primitive_ty) = try_enum {
                    let range = match primitive_ty.to_string().as_str() {
                        "u8" => parse_range::<u8>(bits, &name)?,
                        "u16" => parse_range::<u16>(bits, &name)?,
                        _ => {
                            return Err((
                                "Only u8/u16 enums are supported at the moment".to_string(),
                                primitive_ty.span(),
                            ))
                        }
                    };
                    FieldTy::TryEnum {
                        range,
                        primitive_ty,
                    }
                } else {
                    return Err((
                        format!("Unknown type {:?}, only `bool` is supported", field.ty),
                        name.span(),
                    ));
                }
            }
        };

        result.push(Field {
            name,
            readable,
            writable,
            docs,
            ty,
            ty_info,
        });
    }
    Ok(result)
}

fn parse_range<T>(
    bits: Option<Vec<TokenTree>>,
    name: &Ident,
) -> Result<Range<usize>, (String, Span)> {
    parse_range_ty(
        bits,
        name,
        std::mem::size_of::<T>() * 8,
        std::any::type_name::<T>(),
    )
}
fn parse_range_ty(
    bits: Option<Vec<TokenTree>>,
    name: &Ident,
    max_size: usize,
    ty_name: &str,
) -> Result<Range<usize>, (String, Span)> {
    if let Some(bits) = bits {
        match bits.as_slice() {
            [TokenTree::Literal(high), TokenTree::Punct(colon), TokenTree::Literal(low)]
                if colon.as_char() == ':' =>
            {
                let high_span = high.span();
                let high = high
                    .to_string()
                    .parse()
                    .map_err(|_| ("Invalid value".to_string(), high.span()))?;
                let low = low
                    .to_string()
                    .parse()
                    .map_err(|_| ("Invalid value".to_string(), low.span()))?;
                if low > high || low + max_size <= high {
                    return Err((format!("bit range too big for {}", ty_name), high_span));
                }
                if low / 32 != high / 32 {
                    return Err((
                        "bit range overlaps u32 boundary, this is currently not supported"
                            .to_string(),
                        high_span,
                    ));
                }
                Ok(low..high)
            }
            [token, ..] => Err((
                "Invalid value, should be `high:low`".to_string(),
                token.span(),
            )),
            _ => Err(("Missing `bits = high:low`".to_string(), name.span())),
        }
    } else {
        Err(("Missing `bits = high:low` field".to_string(), name.span()))
    }
}

pub struct Field<'a> {
    pub name: Ident,
    pub readable: bool,
    pub writable: bool,
    pub docs: Vec<&'a Attribute>,

    pub ty: &'a Type,
    pub ty_info: FieldTy,
}

pub enum FieldTy {
    Bool {
        bit: usize,
        reset: Option<bool>,
    },
    U8 {
        range: Range<usize>,
        reset: Option<u8>,
    },
    U16 {
        range: Range<usize>,
        reset: Option<u16>,
    },
    TryEnum {
        range: Range<usize>,
        primitive_ty: Ident,
    },
    UnsafeEnum {
        range: Range<usize>,
        primitive_ty: Ident,
    },
}
