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
    for (i, opt) in input_msg_options.iter().enumerate() {
        let option_name = &opt.option_name;
        let expr = &opt.expr;

        let token = match option_name.to_string().to_lowercase().as_str() {
            "tts" => quote! { .tts( #expr ) },
            "add_file" => quote! { .add_file( #expr ) },
            "add_files" => quote! { .add_files( #expr ) },
            "files" => quote! { .files( #expr ) },
            "content" => quote! { .content( #expr ) },
            "add_embed" => quote! { .add_embed( #expr ) },
            "add_embeds" => quote! { .add_embeds( #expr ) },
            "embed" => quote! { .embed( #expr ) },
            "embeds" => quote! { .embeds( #expr ) },
            "allowed_mentions" => quote! { .allowed_mentions( #expr ) },
            "flags" => quote! { .flags( #expr ) },
            "ephemeral" => quote! { .ephemeral( #expr ) },
            "components" => quote! { .components( #expr ) },
            "poll" => quote! { .poll( #expr ) },
            "button" => quote! { .button( #expr ) },
            "select_menu" => quote! { .select_menu( #expr ) },

            _ => {
                return Err(Error::new(
                    parsed.get(i).unwrap().span(),
                    format!("Unknown option: {}", option_name),
                ));
            }
        };

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
