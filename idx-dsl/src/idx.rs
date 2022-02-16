use syn::Ident;

mod options {
    use super::parser::kw;
    use std::ops::Deref;
    use syn::Ident;

    #[derive(Debug)]
    pub struct BaseOpsOpt {
        pub kw: kw::base_ops, // for error diagnostics
    }

    #[derive(Debug)]
    pub struct OffsetOpt {
        pub kw: kw::offset, // for error diagnostics
        pub offset: Ident,
    }

    // This is a wrapper around Option that I can add methods to.
    // It is an optional option, i.e., one that doesn't have to be
    // provided.
    #[derive(Debug)]
    pub struct OptionalOpt<T>(pub Option<T>);
    impl<T> Default for OptionalOpt<T> {
        fn default() -> Self {
            OptionalOpt(None)
        }
    }
    impl<T> Deref for OptionalOpt<T> {
        type Target = Option<T>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}

#[derive(Default, Debug)]
pub struct IdxTypeOptions {
    pub base_ops: options::OptionalOpt<options::BaseOpsOpt>,
    pub offset: options::OptionalOpt<options::OffsetOpt>,
}

#[derive(Debug)]
pub struct IdxType {
    pub name: Ident,
    pub wrap_type: Ident,
}

mod parser {
    use super::options as opt;
    use super::{IdxType, IdxTypeOptions};
    use crate::err;
    use syn::{
        parse::{Parse, ParseStream},
        punctuated::Punctuated,
        Error, Ident, Result, Token,
    };

    pub mod kw {
        use syn::custom_keyword;
        custom_keyword!(base_ops);
        custom_keyword!(offset);
    }

    #[derive(Debug)]
    enum IdxTypeOption {
        BaseOps(kw::base_ops),
        Offset(kw::offset, Ident),
    }

    impl Parse for IdxTypeOption {
        fn parse(input: ParseStream) -> Result<Self> {
            if input.peek(kw::base_ops) {
                let kw: kw::base_ops = input.parse()?;
                Ok(IdxTypeOption::BaseOps(kw))
            } else if input.peek(kw::offset) {
                let kw: kw::offset = input.parse()?;
                let _: Token![=] = input.parse()?;
                let offset = input.parse()?;
                Ok(IdxTypeOption::Offset(kw, offset))
            } else {
                Err(input.error("Unknown option"))
            }
        }
    }

    pub trait UpdateErr {
        fn err(&self, upd: &Self) -> Error;
    }
    impl UpdateErr for opt::BaseOpsOpt {
        fn err(&self, upd: &Self) -> Error {
            err::redundant_error(self.kw, upd.kw)
        }
    }
    impl UpdateErr for opt::OffsetOpt {
        fn err(&self, upd: &Self) -> Error {
            err::redundant_error(self.kw, upd.kw)
        }
    }
    impl<T: UpdateErr> opt::OptionalOpt<T> {
        fn update(&mut self, upd: T) -> Result<()> {
            let existing: &Option<T> = &self.0;
            match &existing {
                &Some(existing) => Err(existing.err(&upd)),
                _ => {
                    self.0 = Some(upd);
                    Ok(())
                }
            }
        }
    }

    impl IdxTypeOptions {
        /// Merge an option into the collection of options. We could do this functional, but it
        /// is more work, and there is nothing wrong with some imperative programming from time
        /// to time. Updating just the option we want is easier with a mutable self.
        fn merge_opt(&mut self, opt: &IdxTypeOption) -> syn::Result<&mut Self> {
            match opt {
                IdxTypeOption::BaseOps(kw) => {
                    self.base_ops.update(opt::BaseOpsOpt { kw: *kw })?;
                }
                IdxTypeOption::Offset(kw, ident) => {
                    self.offset.update(opt::OffsetOpt {
                        kw: *kw,
                        offset: ident.clone(),
                    })?;
                }
            };
            Ok(self)
        }
    }

    impl Parse for IdxTypeOptions {
        fn parse(input: ParseStream) -> Result<Self> {
            let options: Punctuated<IdxTypeOption, Token![,]> =
                input.parse_terminated(IdxTypeOption::parse)?;
            let mut opts = IdxTypeOptions::default();
            for opt in options.iter() {
                opts.merge_opt(opt)?;
            }
            Ok(opts)
        }
    }

    impl Parse for IdxType {
        fn parse(input: ParseStream) -> Result<Self> {
            let _: Token![type] = input.parse()?;
            let name = input.parse()?;
            let _: Token![=] = input.parse()?;
            let wrap_type = input.parse()?; // FIXME: type check
            let _: Token![;] = input.parse()?;
            Ok(IdxType { name, wrap_type })
        }
    }
}

pub mod codegen {
    use super::options as opt;
    use super::{IdxType, IdxTypeOptions};
    use crate::{ops, wrap_type::gen_wrap_type};
    use proc_macro2::TokenStream;
    use quote::{quote, quote_spanned};
    use syn::Result;

    // Code generation for options
    pub trait OptCodeGen {
        fn code_gen(&self, t: &IdxType) -> Result<TokenStream>;
    }

    impl<T: OptCodeGen> OptCodeGen for opt::OptionalOpt<T> {
        fn code_gen(&self, t: &IdxType) -> Result<TokenStream> {
            match &self.0 {
                None => Ok(quote!()),
                Some(wrapped) => wrapped.code_gen(t),
            }
        }
    }

    impl OptCodeGen for opt::BaseOpsOpt {
        fn code_gen(&self, t: &IdxType) -> Result<TokenStream> {
            let opt::BaseOpsOpt { kw } = self;
            let IdxType { name, wrap_type } = t;

            let span = kw.span;
            let dsl_code = quote_spanned! {span=>
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

            syn::parse2::<ops::Ops>(dsl_code)?.code_gen()
        }
    }

    impl OptCodeGen for opt::OffsetOpt {
        fn code_gen(&self, t: &IdxType) -> Result<TokenStream> {
            let opt::OffsetOpt { offset, .. } = self;
            let IdxType { name, .. } = t;

            let span = offset.span();
            let dsl_code = quote_spanned! {span=>
                    #name - #name => #offset,
                    #name + #offset => #name,
                    #offset + #name => #name,
                    #name += #offset,
                    #name -= #offset,
            };

            syn::parse2::<ops::Ops>(dsl_code)?.code_gen()
        }
    }

    pub fn emit_idx_type(options: &IdxTypeOptions, idx_type: &IdxType) -> Result<TokenStream> {
        let IdxType { name, wrap_type } = idx_type;

        let typedef = gen_wrap_type(name, wrap_type);
        let base_ops = options.base_ops.code_gen(idx_type)?;
        let offset_ops = options.offset.code_gen(idx_type)?;

        Ok(quote! {
            #typedef
            #base_ops
            #offset_ops
        }
        .into())
    }
}
