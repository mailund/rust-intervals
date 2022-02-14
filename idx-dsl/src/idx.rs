use syn::Ident;

#[derive(Debug)]
pub struct IdxType {
    pub name: Ident,
    pub wrap_type: Ident,
}

#[allow(dead_code)] // the fields are seen in the macro but the analyser doesn't see that
mod parser {
    use super::IdxType;
    use syn::parse::{Parse, ParseStream};
    use syn::{Result, Token};

    impl Parse for IdxType {
        fn parse(input: ParseStream) -> Result<Self> {
            let _ = input.parse::<Token![type]>()?;
            let name = input.parse()?;
            let _ = input.parse::<Token![=]>();
            let wrap_type = input.parse()?;
            let _ = input.parse::<Token![;]>()?;
            Ok(IdxType { name, wrap_type })
        }
    }
}
