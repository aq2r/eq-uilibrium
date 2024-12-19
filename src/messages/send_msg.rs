use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{ParseStream, Parser},
    parse2,
    punctuated::Punctuated,
    spanned::Spanned,
    Error, Expr, Ident, Path, Result, Token,
};

use super::parse::{eq_channel_parse, parse_ident_dot, parse_option_args, TokenOrErrToken};

pub fn _send_msg(tokens: TokenStream) -> TokenStream {
    send_msg_parse
        .parse2(tokens)
        .unwrap_or_else(Error::into_compile_error)
}

fn send_msg_parse(input: ParseStream) -> Result<TokenStream> {
    let channel_tokens = match eq_channel_parse(input, "Expected ChannelId")? {
        TokenOrErrToken::Token(token_stream) => token_stream,
        TokenOrErrToken::ErrToken(token_stream) => return Ok(token_stream),
    };

    let _: Token![,] = input.parse()?;

    let expr_tokens = parse_ident_dot(input)?;

    let _: Result<Token![,]> = input.parse();

    let parsed = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;
    let option_tokens = parse_option_args(parsed);

    Ok(quote! {
        #channel_tokens . send_message ( #(#expr_tokens)* ,
            serenity::all::CreateMessage::new() #( #option_tokens )*
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
        let token = _send_msg(quote! { msg.channel_id, &ctx.http }).to_string();
        dbg!(&token);
    }
}
