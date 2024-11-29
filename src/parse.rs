use syn::{
    parse::{Parse, ParseStream},
    Expr, Ident, Result, Token,
};

#[derive(Debug)]
pub struct MsgOptionArg {
    pub option_name: Ident,
    _equal: Token![=],
    pub expr: Expr,
}

impl Parse for MsgOptionArg {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(MsgOptionArg {
            option_name: input.parse()?,
            _equal: input.parse()?,
            expr: input.parse()?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use quote::quote;
    use syn::parse2;

    #[ignore]
    #[test]
    fn dbg_parse() {
        let result = parse2::<MsgOptionArg>(quote! { some = abc }).unwrap();
        dbg!(result);
    }
}
