use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::DeriveInput;

mod generate;
mod parse;

pub fn derive(input: DeriveInput) -> Result<TokenStream, (String, Span)> {
    let parse::StructAttributes { offset, inner } = parse::get_struct_attributes(&input)?;
    let fields = parse::get_struct_fields(&input)?;
    let ident = &input.ident;
    let inner: TokenStream = inner.parse().unwrap();
    let module_name = Ident::new(&ident.to_string().to_lowercase(), ident.span());
    let (read_types, r_methods) = generate::read_types_and_methods(&fields);
    let (write_default, write_types, w_methods) =
        generate::write_default_types_and_methods(&inner, &fields);

    Ok(quote! {
        pub mod #module_name {
            pub struct R(pub(super) #inner);
            impl R {
                #(#r_methods)*
            }
            #( #read_types )*

            pub struct W(pub(super) #inner);

            #write_default

            impl W {
                #(#w_methods)*
            }

            #( #write_types )*
        }

        impl super::Peripheral for #ident {
            type R = #module_name::R;
            type W = #module_name::W;

            unsafe fn read(base_address: core::ptr::NonNull<()>) -> Self::R {
                use core::num::NonZeroUsize;
                let bytes: #inner = unsafe { core::ptr::read_volatile(
                    base_address.map_addr(|addr| {
                        NonZeroUsize::new(addr.get() + #offset).unwrap()
                    }).cast().as_ptr()
                ) };
                #module_name::R(bytes)
            }

            unsafe fn write(base_address: core::ptr::NonNull<()>, value: Self::W) {
                use core::num::NonZeroUsize;
                let bytes = value.0;
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
