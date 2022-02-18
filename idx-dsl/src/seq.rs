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
        custom_keyword!(index);
    }

    impl Parse for SeqType {
        fn parse(input: ParseStream) -> Result<Self> {
            let _: Token![type] = input.parse()?;
            let name: Ident = input.parse()?;

            let _: Token![=] = input.parse()?;

            let of_type;
            syn::bracketed!(of_type in input);
            let of_type: Ident = of_type.parse()?;

            let _: Token![;] = input.parse()?;
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
