use syn::{BinOp, Ident};

#[derive(Debug)]
pub enum Op {
    BinOp(StructBinOp),
    AssignOp(StructAssignOp),
}

#[derive(Debug)]
pub struct StructBinOp {
    pub lhs: Ident,
    pub op: BinOp,
    pub rhs: Ident,
    pub res: Ident,
}

#[derive(Debug)]
pub struct StructAssignOp {
    pub lhs: Ident,
    pub op: BinOp,
    pub rhs: Ident,
}

#[derive(Debug)]
pub struct Ops {
    pub ops: Vec<Op>,
}

#[allow(dead_code)] // the fields are seen in the macro but the analyser doesn't see that
mod parser {
    use super::{Op, Ops, StructAssignOp, StructBinOp};
    use syn::{
        parse::{Parse, ParseStream},
        punctuated::Punctuated,
        spanned::Spanned,
        BinOp, Error, Ident, Result, Token,
    };
    use BinOp::*;

    impl Parse for Ops {
        fn parse(input: ParseStream) -> Result<Self> {
            let ops: Punctuated<Op, Token![,]> = input.parse_terminated(Op::parse)?;
            let ops: Vec<_> = ops.into_iter().collect();
            Ok(Ops { ops })
        }
    }

    impl Op {
        fn parse_binop(lhs: Ident, op: BinOp, input: ParseStream) -> Result<Self> {
            let rhs = input.parse()?;
            let _: syn::token::FatArrow = input.parse()?;
            let res = input.parse()?;
            Ok(Op::BinOp(StructBinOp { lhs, op, rhs, res }))
        }

        fn parse_assignop(lhs: Ident, op: BinOp, input: ParseStream) -> Result<Self> {
            let rhs = input.parse()?;
            Ok(Op::AssignOp(StructAssignOp { lhs, op, rhs }))
        }
    }

    impl Parse for Op {
        fn parse(input: ParseStream) -> Result<Self> {
            let lhs = input.parse()?;
            let op = input.parse()?;
            match op {
                // We allow +, -, * and / here and not the other bin-ops
                // NOTE: This might change in the future.
                Add(_) | Sub(_) | Mul(_) | Div(_) => Op::parse_binop(lhs, op, input),
                AddEq(_) | SubEq(_) | MulEq(_) | DivEq(_) => Op::parse_assignop(lhs, op, input),
                _ => Err(Error::new(
                    op.span(),
                    "Only +, -, *, / or +=, -=, *=, or /= allowed.",
                )),
            }
        }
    }
}

pub mod codegen {
    use super::{Op, StructAssignOp, StructBinOp};
    use crate::hygiene::idx_types;
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::spanned::Spanned;
    use syn::{BinOp, Error, Result};

    pub fn emit_op(op: &Op) -> Result<TokenStream> {
        match &op {
            &Op::AssignOp(op) => emit_assignop(&op),
            &Op::BinOp(op) => emit_binop(&op),
        }
    }

    struct BinopTrait {
        trait_name: TokenStream,
        method_name: TokenStream,
    }

    #[rustfmt::skip]
    fn get_binop_trait(op: &BinOp) -> Result<BinopTrait> {
        use BinOp::*;
        match &op {
            Add(_) => Ok(BinopTrait { trait_name: quote!(std::ops::Add), 
                                      method_name: quote!(add), }),
            Sub(_) => Ok(BinopTrait { trait_name: quote!(std::ops::Sub), 
                                      method_name: quote!(sub), }),
            Mul(_) => Ok(BinopTrait { trait_name: quote!(std::ops::Mul), 
                                      method_name: quote!(mul), }),
            Div(_) => Ok(BinopTrait { trait_name: quote!(std::ops::Div), 
                                      method_name: quote!(div), }),
            
            AddEq(_) => Ok(BinopTrait { trait_name: quote!(std::ops::AddAssign), 
                                        method_name: quote!(add_assign), }),
            SubEq(_) => Ok(BinopTrait { trait_name: quote!(std::ops::SubAssign),
                                        method_name: quote!(sub_assign), }),
            MulEq(_) => Ok(BinopTrait { trait_name: quote!(std::ops::MulAssign), 
                                        method_name: quote!(mul_assign), }),
            DivEq(_) => Ok(BinopTrait { trait_name: quote!(std::ops::DivAssign), 
                                        method_name: quote!(div_assign), }),

            _ => Err(Error::new(op.span(), "Operator currently not supported.")),
        }
    }

    fn emit_assignop(op: &StructAssignOp) -> Result<TokenStream> {
        let StructAssignOp { lhs, op, rhs } = op;
        let BinopTrait {
            trait_name,
            method_name,
        } = get_binop_trait(&op)?;

        let crate_type_traits = idx_types(Some(quote!(type_traits)));
        let op_trait = quote! {
            impl #trait_name<#rhs> for #lhs
            {
                fn #method_name(&mut self, rhs: #rhs) {
                    let rhs = #crate_type_traits::cast_underlying::<#rhs,#lhs>(rhs);
                    self.0 #op rhs;
                }
            }
        };
        Ok(op_trait)
    }

    fn emit_binop(op: &StructBinOp) -> Result<TokenStream> {
        let StructBinOp { lhs, op, rhs, res } = op;
        let BinopTrait {
            trait_name,
            method_name,
        } = get_binop_trait(&op)?;

        let crate_type_traits = idx_types(Some(quote!(type_traits)));
        let op_trait = quote! {
            impl #trait_name<#rhs> for #lhs
            {
                type Output = #res;
                fn #method_name(self, rhs: #rhs) -> Self::Output {
                    let lhs = #crate_type_traits::cast_underlying::<#lhs,#res>(self);
                    let rhs = #crate_type_traits::cast_underlying::<#rhs,#res>(rhs);
                    (lhs #op rhs).into()
                }
            }
        };
        Ok(op_trait)
    }

    pub fn emit_ops(ops: &super::Ops) -> Result<TokenStream> {
        let super::Ops { ops } = ops;
        let gen_ops = ops
            .into_iter()
            .map(|op| emit_op(&op))
            .collect::<Result<TokenStream>>()?;
        Ok(gen_ops)
    }

    impl super::Ops {
        pub fn code_gen(&self) -> Result<TokenStream> {
            emit_ops(self)
        }
    }
}
