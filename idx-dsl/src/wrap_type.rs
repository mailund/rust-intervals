use crate::hygiene::idx_types;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn gen_wrap_type(name: &Ident, wrap_type: &Ident) -> TokenStream {
    let type_traits = idx_types(Some(quote!(type_traits)));
    quote! {
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
    }
}
