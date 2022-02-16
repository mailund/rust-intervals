#![feature(proc_macro_quote)]
#![feature(proc_macro_diagnostic)]

mod hygiene;
mod idx;
mod offset;
mod ops;
mod wrap_type;

mod err {
    use quote::ToTokens;
    use syn::Error;

    pub fn redundant_error<T: ToTokens>(a: T, b: T) -> Error {
        let mut error = Error::new_spanned(b, "redundant attribute argument");
        error.combine(Error::new_spanned(a, "note: first one here"));
        error
    }
}

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Define sequence types.
#[proc_macro_attribute]
pub fn seq_type(_attr: TokenStream, _input: TokenStream) -> TokenStream {
    quote::quote!().into()
}

/// Define offset types.
#[proc_macro_attribute]
pub fn offset_type(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let offset_type = parse_macro_input!(input as offset::OffsetType);
    offset::codegen::emit_offset_type(&offset_type)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Define index types.
#[proc_macro_attribute]
pub fn idx_type(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let options = parse_macro_input!(attrs as idx::IdxTypeOptions);
    let idx_type = parse_macro_input!(input as idx::IdxType);
    idx::codegen::emit_idx_type(&options, &idx_type)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Define operations we can do on our new types.
#[proc_macro]
pub fn def_ops(input: TokenStream) -> TokenStream {
    let ops = parse_macro_input!(input as ops::Ops);
    ops::codegen::emit_ops(&ops)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
