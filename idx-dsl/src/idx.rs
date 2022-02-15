use syn::Ident;

#[derive(Default, Debug)]
pub struct IdxTypeOptions {
    pub base_ops: Option<parser::kw::base_ops>,
    pub offset: Option<Ident>,
}

#[derive(Debug)]
pub struct IdxType {
    pub name: Ident,
    pub wrap_type: Ident,
}

mod parser {
    use super::{IdxType, IdxTypeOptions};
    use quote::ToTokens;
    use syn::{
        parse::{Parse, ParseStream},
        punctuated::Punctuated,
        Ident, Result, Token,
    };

    pub mod kw {
        use syn::custom_keyword;
        custom_keyword!(base_ops);
        custom_keyword!(offset);
    }
    enum IdxTypeOption {
        BaseOps(kw::base_ops), // We keep the Ident for error diagnostics
        Offset(Ident),
    }
    impl IdxTypeOption {
        fn to_options(&self) -> IdxTypeOptions {
            match &self {
                &IdxTypeOption::BaseOps(kw) => IdxTypeOptions {
                    base_ops: Some(kw.clone()),
                    offset: None,
                },
                &IdxTypeOption::Offset(ident) => IdxTypeOptions {
                    base_ops: None,
                    offset: Some(ident.clone()),
                },
            }
        }
    }
    impl Parse for IdxTypeOption {
        fn parse(input: ParseStream) -> Result<Self> {
            if input.peek(kw::base_ops) {
                let ident: kw::base_ops = input.parse()?;
                Ok(IdxTypeOption::BaseOps(ident))
            } else if input.peek(kw::offset) {
                let _: kw::offset = input.parse()?;
                let _: Token![=] = input.parse()?;
                let offset_type = input.parse()?;
                Ok(IdxTypeOption::Offset(offset_type))
            } else {
                Err(input.error("Unknown option"))
            }
        }
    }

    impl IdxTypeOptions {
        fn merge(self, other: IdxTypeOptions) -> syn::Result<Self> {
            fn either<T: ToTokens>(a: Option<T>, b: Option<T>) -> syn::Result<Option<T>> {
                match (a, b) {
                    (None, None) => Ok(None),
                    (Some(val), None) | (None, Some(val)) => Ok(Some(val)),
                    (Some(a), Some(b)) => {
                        let mut error = syn::Error::new_spanned(b, "redundant attribute argument");
                        error.combine(syn::Error::new_spanned(a, "note: first one here"));
                        Err(error)
                    }
                }
            }

            Ok(Self {
                base_ops: either(self.base_ops, other.base_ops)?,
                offset: either(self.offset, other.offset)?,
            })
        }
    }

    impl Parse for IdxTypeOptions {
        fn parse(input: ParseStream) -> Result<Self> {
            let options: Punctuated<IdxTypeOption, Token![,]> =
                input.parse_terminated(IdxTypeOption::parse)?;
            let opts: Result<Self> = options
                .iter()
                .try_fold(IdxTypeOptions::default(), |current, new| {
                    current.merge(new.to_options())
                });
            opts
        }
    }

    impl Parse for IdxType {
        fn parse(input: ParseStream) -> Result<Self> {
            let _: Token![type] = input.parse()?;
            let name = input.parse()?;
            let _: Token![=] = input.parse()?;
            let wrap_type = input.parse()?;
            let _: Token![;] = input.parse()?;
            Ok(IdxType { name, wrap_type })
        }
    }
}

pub mod codegen {
    use super::{IdxType, IdxTypeOptions};
    use crate::{hygiene::idx_types_id, ops};
    use proc_macro2::TokenStream;
    use quote::{quote, quote_spanned};
    use syn::Result;

    pub fn emit_idx_type(options: &IdxTypeOptions, idx_type: &IdxType) -> Result<TokenStream> {
        let IdxType { name, wrap_type } = idx_type;
        let type_traits = idx_types_id(quote!(type_traits));

        let typedef = quote::quote! {
            /* FIXME: need to check type
            extern crate static_assertions;
            static_assertions::assert_impl_all!(#wrap_type, #type_traits::CastType);
            */

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
        };

        let base_ops = match options.base_ops {
            Some(token) => {
                let span = token.span;
                let code = quote_spanned! {span=>
                    #name + #wrap_type => #name,
                    #wrap_type + #name => #name,
                    #name - #wrap_type => #name,
                    #wrap_type - #name => #name,
                    #name * #wrap_type => #name,
                    #wrap_type * #name => #name,
                    #name / #wrap_type => #name,
                    #wrap_type / #name => #name,
                    #name += #wrap_type,
                    #name -= #wrap_type,
                    #name *= #wrap_type,
                    #name /= #wrap_type,
                };
                syn::parse2::<ops::Ops>(code)?
            }
            None => ops::Ops { ops: vec![] },
        };
        let base_ops = ops::codegen::emit_ops(&base_ops)?;

        let offset_ops = match &options.offset {
            Some(offset) => {
                let span = offset.span();
                let code = quote_spanned! {span=>
                    #name - #name => #offset,
                    #name + #offset => #name,
                    #offset + #name => #name,
                    #name += #offset,
                    #name -= #offset,
                };
                syn::parse2::<ops::Ops>(code)?
            }
            None => ops::Ops { ops: vec![] },
        };
        let offset_ops = ops::codegen::emit_ops(&offset_ops)?;

        Ok(quote! {
            #typedef
            #base_ops
            #offset_ops
        }
        .into())
    }
}
