#![feature(proc_macro_quote)]
#![feature(proc_macro_diagnostic)]

mod hygiene {
    use proc_macro2::{Span, TokenStream};
    use proc_macro_crate::{crate_name, FoundCrate};
    use quote::quote;
    use syn::Ident;

    pub fn idx_types_id(item: TokenStream) -> TokenStream {
        let found_crate = crate_name("idx-types").expect("idx-types is present in `Cargo.toml`");

        match found_crate {
            FoundCrate::Itself => quote!(crate::#item),
            FoundCrate::Name(name) => {
                let crate_ = Ident::new(&name, Span::call_site());
                quote!( #crate_::#item )
            }
        }
        .into()
    }
}

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
