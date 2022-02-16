#![feature(proc_macro_quote)]
#![feature(proc_macro_diagnostic)]

mod hygiene;
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
pub fn idx_type(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let options = parse_macro_input!(attrs as idx::IdxTypeOptions);
    //parse_macro_input!(attrs as idx::IdxTypeOptions);
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
