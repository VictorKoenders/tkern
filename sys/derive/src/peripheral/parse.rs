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
                let reset =
                    if let Some(reset) = reset {
                        Some(reset.to_string().parse().map_err(|_| {
                            ("Invalid reset value for u8".to_string(), reset.span())
                        })?)
                    } else {
                        None
                    };
                let range = if let Some(bits) = bits {
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
                            if low > high || low + 8 <= high {
                                return Err(("bit range too big for u8".to_string(), high_span));
                            }
                            if low / 32 != high / 32 {
                                return Err(("bit range overlaps u32 boundary, this is currently not supported".to_string(), high_span));
                            }
                            low..high
                        }
                        [token, ..] => {
                            return Err((
                                "Invalid value, should be `high:low`".to_string(),
                                token.span(),
                            ));
                        }
                        _ => {
                            return Err(("Missing `bits = high:low`".to_string(), name.span()));
                        }
                    }
                } else {
                    return Err(("Missing `bits = high:low` field".to_string(), name.span()));
                };
                FieldTy::U8 { range, reset }
            }
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
            docs,
            ty,
            ty_info,
        });
    }
    Ok(result)
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
}
