use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    Error, Expr, Ident, Result, Token,
};

pub enum TokenOrErrToken {
    Token(TokenStream),
    ErrToken(TokenStream),
}

pub fn parse_ident_dot(input: ParseStream) -> Result<Vec<TokenStream>> {
    let mut items = Vec::new();

    while !input.is_empty() {
        if input.peek(Token![.]) {
            let dot: Token![.] = input.parse()?;
            items.push(quote! { #dot });
        } else if input.peek(Ident) {
            let ident: Ident = input.parse()?;
            items.push(quote! { #ident });
        } else if input.peek(Token![&]) {
            let and: Token![&] = input.parse()?;
            items.push(quote! { #and });
        } else {
            break;
        }
    }

    Ok(items)
}

pub fn eq_channel_parse(input: ParseStream, channel_err_msg: &str) -> Result<TokenOrErrToken> {
    if input.is_empty() {
        let err_tokens = Error::new(input.span(), channel_err_msg).to_compile_error();
        return Ok(TokenOrErrToken::ErrToken(quote! { #err_tokens }));
    }

    let items = parse_ident_dot(input)?;

    if input.is_empty() {
        let err_tokens = Error::new(input.span(), "Expected Http").to_compile_error();

        return Ok(TokenOrErrToken::ErrToken(quote! {{
            #err_tokens
            let _ = #( #items )* ;
        }}));
    }

    Ok(TokenOrErrToken::Token(quote! { #( #items )* }))
}

pub fn parse_option_args(parsed: Punctuated<Expr, Token![,]>) -> Vec<TokenStream> {
    let mut input_msg_options = vec![];
    for expr in parsed {
        match parse2::<MsgOptionArg>(quote! { #expr }) {
            Ok(arg) => {
                let option_name = arg.option_name;
                let option_expr = arg.expr;

                input_msg_options.push(quote! {
                    . #option_name ( #option_expr )
                })
            }
            Err(_) => {
                input_msg_options.push(quote! {
                    . #expr ()
                });
            }
        }
    }

    input_msg_options
}

#[derive(Debug)]
pub struct MsgOptionArg {
    pub option_name: Ident,
    _equal: Token![=],
    pub expr: Expr,
}

impl MsgOptionArg {}

impl Parse for MsgOptionArg {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(MsgOptionArg {
            option_name: input.parse()?,
            _equal: input.parse()?,
            expr: input.parse()?,
        })
    }
}
