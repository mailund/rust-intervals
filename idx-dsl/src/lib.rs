#![feature(proc_macro_quote)]

use proc_macro::TokenStream;
use syn::{parse_macro_input, Ident, BinOp, Result, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

#[allow(dead_code)] // the fields are seen in the macro but the analyser doesn't see that
mod parser {
    use super::*;

    #[derive(Debug)]
    pub struct IdxType {
        pub name: Ident,
        pub wrap_type: Ident
    }
    impl Parse for IdxType {
        fn parse(input: ParseStream) -> Result<Self> {
            let _ = input.parse::<Token![type]>()?;
            let name = input.parse()?;
            let _ = input.parse::<Token![<]>()?;
            let wrap_type = input.parse()?;
            let _ = input.parse::<Token![>]>()?;
            Ok(IdxType { name, wrap_type })
        }
    }

    #[derive(Debug)]
    pub struct Ops {
        pub ops: Punctuated<Op, Token![,]>,
    }
    impl Parse for Ops {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(Ops { ops: input.parse_terminated(Op::parse)? })
        }
    }

    #[derive(Debug)]
    pub struct Op {
        pub lhs: Ident,
        pub op: BinOp,
        pub rhs: Ident,
        pub res: Option<Ident>
    }
    impl Parse for Op {
        fn parse(input: ParseStream) -> Result<Self> {
            
            let lhs = input.parse()?;
            let op = input.parse()?;
            let rhs = input.parse()?;
            let res = if input.peek(syn::token::FatArrow) {
                input.parse::<syn::token::FatArrow>()?;
                Some(input.parse()?)
            } else { None };
            
            Ok(Op { lhs, op, rhs, res })
        }
    }
}
use parser::*;


#[proc_macro]
pub fn seq_type(_input: TokenStream) -> TokenStream {
    quote::quote! {
    }.into()
}

#[proc_macro]
pub fn idx_type(input: TokenStream) -> TokenStream {
    let IdxType { name, wrap_type } = parse_macro_input!(input as IdxType);
    quote::quote! {
        #[derive(Debug, Clone, Copy)]
        pub struct #name(pub #wrap_type);
    }.into()
}

#[proc_macro]
pub fn def_ops(input: TokenStream) -> TokenStream {
    let Ops { ops }  = parse_macro_input!(input as Ops);
    // This moves the ops...
    for op in ops.into_iter() {
        println!("{} ?? {}", op.lhs.to_string(), op.rhs.to_string());
    }
    quote::quote! {
    }.into()
}

