pub struct IndexTrait {
    pub span: proc_macro2::Span,
    pub index: syn::Ident,
    pub seq: syn::Ident,
}

mod parser {
    use super::IndexTrait;
    use syn::{
        parse::{Parse, ParseStream},
        Result, Token,
    };

    impl Parse for IndexTrait {
        fn parse(input: ParseStream) -> Result<Self> {
            let span = input.span();
            let index = input.parse()?;
            let _: Token![for] = input.parse()?;
            let seq = input.parse()?;
            Ok(IndexTrait { span, index, seq })
        }
    }
}

pub mod codegen {
    use super::IndexTrait;
    use crate::hygiene::idx_types;
    use proc_macro2::TokenStream;
    use quote::{format_ident, quote, quote_spanned};
    use syn::Result;

    pub fn emit_index_trait(itrait: &IndexTrait) -> Result<TokenStream> {
        let IndexTrait { span, index, seq } = itrait;
        let slice_name = format_ident!("{}Slice", seq);
        let type_traits = idx_types(Some(quote!(type_traits)));

        let code = quote_spanned! {*span=>
            use #type_traits::IndexType;
            use #type_traits::SeqType;

            impl core::ops::Index<#index> for #seq
            {
                type Output = <#seq as SeqType>::Of;
                #[inline]
                fn index(&self, i: #index) -> &Self::Output {
                    &self[i.index(self.len())]
                }
            }

            impl core::ops::IndexMut<#index> for #seq
            {
                #[inline]
                fn index_mut(&mut self, i: #index) -> &mut Self::Output {
                    self.index_mut(i.index(self.len()))
                }
            }

            impl core::ops::Index<core::ops::Range<#index>> for #seq
            {
                type Output = #slice_name;
                fn index(&self, r: core::ops::Range<#index>) -> &Self::Output {
                    self.0[r.start.index(self.len())..r.end.index(self.len())].into()
                }
            }

            impl core::ops::IndexMut<core::ops::Range<#index>> for #seq
            {
                fn index_mut(&mut self, r: core::ops::Range<#index>) -> &mut Self::Output {
                    let from: usize = r.start.index(self.len());
                    let to: usize = r.end.index(self.len());
                    self.index_mut(from..to)
                }
            }
        };
        Ok(code)
    }
}
