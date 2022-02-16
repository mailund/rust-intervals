use crate::hygiene::idx_types;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn gen_wrap_type(name: &Ident, wrap_type: &Ident) -> TokenStream {
    let type_traits = idx_types(Some(quote!(type_traits)));
    let res = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct #name(pub #wrap_type);

        impl std::convert::From<#wrap_type> for #name
        {
            #[inline]
            fn from(t: #wrap_type) -> Self { #name(t) }
        }

        impl #type_traits::CastType for #name {
            type Type = #wrap_type;
            #[inline]
            fn cast<U: #type_traits::NumCast>(self) -> U {
                #type_traits::ncast::<Self::Type, U>(self.0).unwrap()
            }
        }

        /*
        impl std::iter::Step for #name
        {
            fn steps_between(start: &Self, end: &Self) -> Option<usize> {
                match (start.0, end.0) {
                    (i, j) if i > j => None,
                    (i, j) => Some(#name(j.cast::<usize> - i.cast::<usize>)),
                }
            }
            fn forward_checked(start: Self, count: usize) -> Option<Self> {
                let count: #type_traits::ncast::<usize, <Self as #type_traits::CastType>::Type>(count)?;
                Some(#name(start.0 + count))
            }
            fn backward_checked(start: Self, count: usize) -> Option<Self> {
                let count: #type_traits::ncast::<usize, <Self as #type_traits::CastType>::Type>(count)?;
                Some(#name(start.0 - count))
            }
        }
        */
    };
    //println!("{}", res);
    res
}
