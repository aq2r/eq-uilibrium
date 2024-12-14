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

pub fn _send_msg(tokens: TokenStream) -> TokenStream {
    send_msg_parse
        .parse2(tokens)
        .unwrap_or_else(Error::into_compile_error)
}

fn send_msg_parse(input: ParseStream) -> Result<TokenStream> {
    let parsed = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;

    let result_http = match parsed.get(0) {
        Some(expr) => expr.clone(),
        None => return Err(Error::new(parsed.span(), "Http is missing")),
    };

    let result_channelid = match parsed.get(1) {
        Some(expr) => expr.clone(),
        None => return Err(Error::new(parsed.span(), "ChannelId is missing")),
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
        #result_channelid .send_message( #result_http,
            serenity::all::CreateMessage::new() #( #result_options )*
        )
    })
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use quote::quote;

    use super::_send_msg;

    #[ignore]
    #[test]
    fn dbg() {
        let token = _send_msg(quote! { &ctx.http, channel_id, content = abc }).to_string();
        dbg!(&token);
    }

    #[test]
    fn test_send_msg() {
        let token_1 =
            _send_msg(quote! { &ctx.http, channel_id, content = _1,embed = _2 }).to_string();

        let token_2 = quote! {
            channel_id .send_message( &ctx.http,
            serenity::all::CreateMessage::new() .content(_1) .embed(_2)
        )}
        .to_string();

        assert_eq!(token_1, token_2)
    }

    #[test]
    fn unknown_option() {
        let token = _send_msg(quote! { &ctx, channel_id, option1 = abc }).to_string();
        let token2 =
            quote! { :: core :: compile_error ! { "Unknown option: option1" } }.to_string();
        assert_eq!(token, token2)
    }

    #[test]
    fn expected_option_format() {
        let token = _send_msg(quote! { &ctx, channel_id, option1 = abc, abcde }).to_string();
        let token2 = quote! { :: core :: compile_error ! { "Not in the form of an option. \ne.g. : content = \"Hello, world!\"" } }.to_string();
        assert_eq!(token, token2)
    }

    #[test]
    fn expected_context() {
        let token = _send_msg(quote! {}).to_string();
        let token2 = quote! { :: core :: compile_error ! { "Http is missing" } }.to_string();
        assert_eq!(token, token2)
    }

    #[test]
    fn expected_channelid() {
        let token = _send_msg(quote! { ctx }).to_string();
        let token2 = quote! { :: core :: compile_error ! { "ChannelId is missing" } }.to_string();
        assert_eq!(token, token2)
    }
}
