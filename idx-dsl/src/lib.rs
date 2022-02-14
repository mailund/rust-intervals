#![feature(proc_macro_quote)]
#![feature(proc_macro_diagnostic)]

mod idx;
mod ops;

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Define sequence types.
#[proc_macro_attribute]
pub fn seq_type(_attr: TokenStream, _input: TokenStream) -> TokenStream {
    quote::quote! {}.into()
}

/// Define index types.
#[proc_macro_attribute]
pub fn idx_type(_attr: TokenStream, input: TokenStream) -> TokenStream {
    //let attr = parse_macro_input!(attr as syn::AttributeArgs);
    //println!("Attr: {:#?}", attr);

    let idx::IdxType { name, wrap_type } = parse_macro_input!(input as idx::IdxType);

    let typedef = quote::quote! {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct #name(pub #wrap_type);

        impl std::convert::From<#wrap_type> for #name
        {
            #[inline]
            fn from(t: #wrap_type) -> Self {
                #name(t)
            }
        }

        impl idx_types::type_traits::CastType for #name {
            type Type = #wrap_type;
            fn cast<U: idx_types::type_traits::NumCast>(self) -> U {
                idx_types::type_traits::cast::<Self::Type, U>(self.0).unwrap()
            }
        }
    };

    typedef.into()
}

/// Define operations we can do on our new types.
#[proc_macro]
pub fn def_ops(input: TokenStream) -> TokenStream {
    let ops = parse_macro_input!(input as ops::Ops);
    ops::codegen::emit_ops(&ops)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
