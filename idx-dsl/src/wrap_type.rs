use crate::hygiene::idx_types;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn gen_wrap_type(name: &Ident, wrap_type: &Ident) -> TokenStream {
    // Where we can find idx_types::type_traits
    let type_traits = idx_types(Some(quote!(type_traits)));

    // Module name for trait implementations
    let impl_mod_name = format!("{}_trait_impl", name).to_lowercase();
    let impl_mod = Ident::new(&impl_mod_name, name.span());

    // Formatting string for Display
    let fmt = format!("{}({{}})", name);

    let code = quote! {

        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct #name(pub #wrap_type);

        // Trait implementations in module, to prevent name clash and so we can
        // use traits. We only implement traits, so we can safely use the module
        // after.
        mod #impl_mod {
            use super::#name;
            use #type_traits::{CastType, NumCast, ncast};
            use std::fmt;

            impl std::convert::From<#wrap_type> for #name
            {
                #[inline]
                fn from(t: #wrap_type) -> Self { #name(t) }
            }

            impl fmt::Display for #name {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, #fmt, self.0)
                }
            }

            impl CastType for #name {
                type Type = #wrap_type;
                #[inline]
                fn cast<U: NumCast>(&self) -> U {
                    ncast::<Self::Type, U>(self.0).unwrap()
                }
            }

            impl std::iter::Step for #name
            {
                fn steps_between(start: &Self, end: &Self) -> Option<usize> {
                    if start.0 > end.0 {
                        None
                    } else {
                        let i: usize = start.cast();
                        let j: usize = end.cast();
                        Some(j - i)
                    }
                }

                fn forward_checked(start: Self, count: usize) -> Option<Self> {
                    let count: <Self as CastType>::Type = count.cast();
                    Some(#name(start.0 + count))
                }

                fn backward_checked(start: Self, count: usize) -> Option<Self> {
                    let count: <Self as CastType>::Type = count.cast();
                    Some(#name(start.0 - count))
                }
            }
        }
        use #impl_mod::*;

    };

    code
}
