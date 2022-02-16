use syn::Ident;

#[derive(Debug)]
pub struct OffsetType {
    pub name: Ident,
    pub wrap_type: Ident,
}

pub mod options {} // no options yet

pub mod parser {
    use super::OffsetType;
    use syn::{
        parse::{Parse, ParseStream},
        Result, Token,
    };

    impl Parse for OffsetType {
        fn parse(input: ParseStream) -> Result<Self> {
            let _: Token![type] = input.parse()?;
            let name = input.parse()?;
            let _: Token![=] = input.parse()?;
            let wrap_type = input.parse()?; // FIXME: type check
            let _: Token![;] = input.parse()?;
            Ok(OffsetType { name, wrap_type })
        }
    }
}

pub mod codegen {
    use super::OffsetType;
    use crate::{ops, wrap_type::gen_wrap_type};
    use proc_macro2::TokenStream;
    use quote::{quote, quote_spanned};
    use syn::Result;

    pub fn emit_offset_type(offset_type: &OffsetType) -> Result<TokenStream> {
        let OffsetType { name, wrap_type } = offset_type;

        let type_code = gen_wrap_type(name, wrap_type);

        // FIXME: I am not a hundred procent sure that these are the right operations
        let span = name.span();
        let ops_code = syn::parse2::<ops::Ops>(quote_spanned! {span=>
            #name + #name => #name,
            #name - #name => #name,
            #name += #name,
            #name -= #name,

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
        })?
        .code_gen()?;

        Ok(quote! {
            #type_code
            #ops_code
        })
    }
}
