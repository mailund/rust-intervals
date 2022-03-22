use syn::Ident;

#[derive(Debug)]
pub struct SeqType {
    pub name: Ident,
    pub of_type: Ident,
}

pub mod options {} // no options yet

pub mod parser {
    use super::SeqType;
    use syn::{
        parse::{Parse, ParseStream},
        Ident, Result, Token,
    };

    impl Parse for SeqType {
        fn parse(input: ParseStream) -> Result<Self> {
            let _: Token![type] = input.parse()?;
            let name: Ident = input.parse()?;

            let _: Token![<] = input.parse()?;
            let of_type: Ident = input.parse()?;
            let _: Token![>] = input.parse()?;

            let _: Token![;] = input.parse()?;

            Ok(SeqType { name, of_type })
        }
    }
}

pub mod codegen {
    use super::SeqType;
    use proc_macro2::TokenStream;
    use quote::{format_ident, quote};
    use syn::Result;

    fn emit_slice(seq_type: &SeqType) -> TokenStream {
        let SeqType { name, of_type } = seq_type;
        let slice_name = format_ident!("{}Slice", name);

        quote!(
            #[derive(Debug)]
            #[repr(transparent)] // Because of this we can soundly cast `&{mut }IdxSlice<T>` to `&{mut }[T]`.
            pub struct #slice_name([#of_type]);
            impl<'a> From<&'a [#of_type]> for &'a #slice_name
            {
                fn from(v: &'a [#of_type]) -> &'a #slice_name {
                    unsafe { &*(v as *const [#of_type] as *const #slice_name) }
                }
            }
            impl<'a> From<&'a mut [#of_type]> for &'a mut #slice_name
            {
                fn from(v: &'a mut [#of_type]) -> &'a mut #slice_name {
                    unsafe { &mut *(v as *mut [#of_type] as *mut #slice_name) }
                }
            }
            impl core::ops::Index<usize> for #slice_name
            {
                type Output = #of_type;
                #[inline]
                fn index(&self, i: usize) -> &Self::Output {
                    &self[i]
                }
            }
            impl core::ops::IndexMut<usize> for #slice_name
            {
                #[inline]
                fn index_mut(&mut self, i: usize) -> &mut Self::Output {
                    &mut self[i]
                }
            }
            impl core::ops::Index<core::ops::Range<usize>> for #slice_name
            {
                type Output = #slice_name;
                fn index(&self, r: core::ops::Range<usize>) -> &Self::Output {
                    self.0[r.start..r.end].into()
                }
            }
            impl core::ops::IndexMut<core::ops::Range<usize>> for #slice_name
            {
                fn index_mut(&mut self, r: core::ops::Range<usize>) -> &mut Self::Output {
                    let vals: &mut [#of_type] = &mut self.0[r.start..r.end];
                    vals.into()
                }
            }
        )
    }

    fn emit_vector(seq_type: &SeqType) -> TokenStream {
        let SeqType { name, of_type } = seq_type;
        let vec_name = name;
        let slice_name = format_ident!("{}Slice", name);
        quote!(
            #[derive(Debug)]
            pub struct #vec_name(pub Vec<#of_type>);
            impl From<Vec<#of_type>> for #vec_name {
                fn from(v: Vec<#of_type>) -> #vec_name {
                    #vec_name(v)
                }
            }
            impl core::ops::Deref for #vec_name {
                type Target = #slice_name;
                fn deref(&self) -> &Self::Target {
                    self.0.as_slice().into()
                }
            }
            impl core::ops::DerefMut for #vec_name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    self.0.as_mut_slice().into()
                }
            }
            impl core::ops::Index<usize> for #vec_name
            {
                type Output = #of_type;
                #[inline]
                fn index(&self, i: usize) -> &Self::Output {
                    &self[i]
                }
            }
            impl core::ops::IndexMut<usize> for #vec_name
            {
                #[inline]
                fn index_mut(&mut self, i: usize) -> &mut Self::Output {
                    &mut self[i]
                }
            }
            impl core::ops::Index<core::ops::Range<usize>> for #vec_name
            {
                type Output = #slice_name;
                fn index(&self, r: core::ops::Range<usize>) -> &Self::Output {
                    self.0[r.start..r.end].into()
                }
            }
            impl core::ops::IndexMut<core::ops::Range<usize>> for #vec_name
            {
                fn index_mut(&mut self, r: core::ops::Range<usize>) -> &mut Self::Output {
                    let vals: &mut [#of_type] = &mut self.0[r.start..r.end];
                    vals.into()
                }
            }
            impl #vec_name {
                pub fn len(&self) -> usize {
                    self.0.len()
                }
            }
        )
    }

    pub fn emit_seq_type(seq_type: &SeqType) -> Result<TokenStream> {
        let vector = emit_vector(seq_type);
        let slice = emit_slice(seq_type);
        let code = quote!(
            #vector
            #slice
        );

        Ok(code)
    }
}
