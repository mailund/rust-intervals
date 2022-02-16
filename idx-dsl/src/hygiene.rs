// Macro injection hygiene

use lazy_static::lazy_static;
use proc_macro2::{Span, TokenStream};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::Ident;

fn lookup_crate(name: &str) -> String {
    match crate_name(name).expect(&("Expected ".to_owned() + name + " in `Cargo.toml`")) {
        FoundCrate::Itself => "crate".to_owned(),
        FoundCrate::Name(name) => name,
    }
}

lazy_static! {
    pub static ref IDX_TYPES_NAME: String = lookup_crate("idx-types");
}

pub fn idx_types(path: Option<TokenStream>) -> TokenStream {
    let idx_types = Ident::new(&IDX_TYPES_NAME, Span::call_site());
    match path {
        None => quote!(#idx_types),
        Some(path) => quote!(#idx_types::#path),
    }
    .into()
}
