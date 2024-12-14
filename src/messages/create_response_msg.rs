use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{ParseStream, Parser},
    parse2,
    punctuated::Punctuated,
    spanned::Spanned,
    Error, Expr, Result, Token,
};

use crate::parse::MsgOptionArg;

pub fn _create_response_msg(tokens: TokenStream) -> TokenStream {
    create_response_msg_parse
        .parse2(tokens)
        .unwrap_or_else(Error::into_compile_error)
}

fn create_response_msg_parse(input: ParseStream) -> Result<TokenStream> {
    let parsed = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;

    let result_http = match parsed.get(0) {
        Some(expr) => expr.clone(),
        None => return Err(Error::new(parsed.span(), "Http is missing")),
    };

    let result_interaction = match parsed.get(1) {
        Some(expr) => expr.clone(),
        None => return Err(Error::new(parsed.span(), "Interaction is missing")),
    };

    let mut input_msg_options = vec![];
    for (i, expr) in parsed.iter().enumerate() {
        // 最初の2つの引数を除く
        if i > 1 {
            match parse2::<MsgOptionArg>(quote! { #expr }) {
                Ok(arg) => input_msg_options.push(arg),
                Err(_) => {
                    return Err(Error::new(
                        expr.span(),
                        "Not in the form of an option. \ne.g. : content = \"Hello, world!\"",
                    ))
                }
            }
        }
    }

    // 最終的に追加する token stream に変換
    let mut result_options = vec![];
    for opt in input_msg_options {
        let option_name = &opt.option_name;
        let expr = &opt.expr;

        let token = quote! { . #option_name ( #expr ) };

        result_options.push(token);
    }

    Ok(quote! {
        #result_interaction .create_response( #result_http,
            serenity::all::CreateInteractionResponse::Message(
                serenity::all::CreateInteractionResponseMessage::new() #( #result_options )*
            )
        )
    })
}
