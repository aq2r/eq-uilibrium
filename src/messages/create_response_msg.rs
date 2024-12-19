use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{ParseStream, Parser},
    punctuated::Punctuated,
    Error, Expr, Result, Token,
};

use super::parse::{eq_channel_parse, parse_ident_dot, parse_option_args, TokenOrErrToken};

pub fn _create_response_msg(tokens: TokenStream) -> TokenStream {
    create_response_msg_parse
        .parse2(tokens)
        .unwrap_or_else(Error::into_compile_error)
}

fn create_response_msg_parse(input: ParseStream) -> Result<TokenStream> {
    let interaction_tokens = match eq_channel_parse(input, "Expected Interaction")? {
        TokenOrErrToken::Token(token_stream) => token_stream,
        TokenOrErrToken::ErrToken(token_stream) => return Ok(token_stream),
    };

    let _: Token![,] = input.parse()?;

    let expr_tokens = parse_ident_dot(input)?;

    let _: Result<Token![,]> = input.parse();

    let parsed = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;
    let option_tokens = parse_option_args(parsed);

    Ok(quote! {
        #interaction_tokens .create_response( #(#expr_tokens)* ,
            serenity::all::CreateInteractionResponse::Message(
                serenity::all::CreateInteractionResponseMessage::new() #( #option_tokens )*
            )
        )
    })
}
