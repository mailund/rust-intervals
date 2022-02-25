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

    pub mod kw {
        use syn::custom_keyword;
        custom_keyword!(of);
    }

    impl Parse for SeqType {
        fn parse(input: ParseStream) -> Result<Self> {
            let _: Token![struct] = input.parse()?;
            let name: Ident = input.parse()?;

            let block;
            syn::braced!(block in input);
            let _: kw::of = block.parse()?;
            let _: Token![:] = block.parse()?;
            let of_type: Ident = block.parse()?;
            let _: Token![,] = block.parse()?;

            Ok(SeqType { name, of_type })
        }
    }
}

pub mod codegen {
    use super::SeqType;
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::Result;

    pub fn emit_seq_type(_seq_type: &SeqType) -> Result<TokenStream> {
        //let SeqType { name, of_type } = seq_type;

        let code = quote!(); // for now

        Ok(code)
    }
}
