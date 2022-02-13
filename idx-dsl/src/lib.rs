#![feature(proc_macro_quote)]



use proc_macro::TokenStream;
use syn::{parse_macro_input, Ident, BinOp, Result, Token, bracketed};
use syn::parse::{Parse, ParseStream};

#[allow(dead_code)] // the fields are seen in the macro but the analyser doesn't see that
mod parser {
    use super::*;

    #[derive(Debug)]
    pub struct IdxType {
        pub name: Ident,
        pub wrap_type: Ident,
        pub ops: Vec<Op>
    }
    impl Parse for IdxType {
        fn parse(input: ParseStream) -> Result<Self> {
            let _ = input.parse::<Token![type]>()?;
            let name = input.parse()?;
            let wt; bracketed!(wt in input);
            let wrap_type = wt.parse()?;

            let mut ops = Vec::<Op>::new();
            // read ops until there is nothing left...
            while let Ok(op) = input.parse::<Op>() {
                ops.push(op);
            }
            
            Ok(IdxType { name, wrap_type, ops })
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
            let lookahead = input.lookahead1();
            
            let lhs = input.parse()?;
            let op = input.parse()?;
            let rhs = input.parse()?;

            println!("do we see =>? {}", lookahead.peek(syn::token::FatArrow));
            println!("do we see =>? {}", lookahead.peek(Token![=>]));
            println!("{:#?}", input.parse::<syn::token::FatArrow>()?);
            println!("I found a res");
            let res = Some(input.parse()?);
            /*
            let res = if lookahead.peek(syn::token::FatArrow) {
                input.parse::<syn::token::FatArrow>()?;
                println!("I found a res");
                Some(input.parse()?)
            } else {println!("I didn't find a res"); None };
            */
            
            Ok(Op { lhs, op, rhs, res })
        }
    }
}
use parser::*;


#[proc_macro]
pub fn idx_type(input: TokenStream) -> TokenStream {
    let IdxType { name, wrap_type, ops } = parse_macro_input!(input as IdxType);
    println!("{:#?}", name);
    println!("{:#?}", wrap_type);
    println!("{:#?}", ops);
    quote::quote! {
    }.into()
}

#[proc_macro]
pub fn seq_type(_input: TokenStream) -> TokenStream {
    quote::quote! {
    }.into()
}

