//! # eq-ilibrium
//!
//! Macros that allow `serenity`'s `send_message` and `create_responce(Message)` to be used like Python keyword arguments

mod messages;
use messages::{create_response_msg::_create_response_msg, send_msg::_send_msg};
use proc_macro::TokenStream;

/// A macro that allows you to input `serenity::all::ChannelId::send_message` as if it were a python keyword argument.
///
/// Enter `Http` as the first argument and the `ChannelId` to which you want to send the message as the second argument.
///
/// ## Examples
///
/// ```ignore
/// /* ----- */
///
/// let embed = CreateEmbed::new().title("embed").description("description");
///
/// eq_uilibrium::send_msg!(&http, channel_id, embed = embed, tts = true).await;
/// // let builder = CreateMessage::new().content("content").tts(true).embed(embed);
/// // let _ = channel_id.send_message(&http, builder).await;
///
/// /* ----- */
#[proc_macro]
pub fn send_msg(tokens: TokenStream) -> TokenStream {
    _send_msg(tokens.into()).into()
}

/// A macro that allows you to input `serenity::all::ChannelId::create_response(Message)` as if it were a python keyword argument.
///
/// Enter `Http` as the first argument and the `Interaction` to which you want to send the message as the second argument.
///
/// ## Examples
///
/// ```ignore
/// /* ----- */
///
/// let embed = CreateEmbed::new().title("embed").description("description");
///
/// eq_uilibrium::create_response_msg!(&http, interaction, embed = embed, tts = true).await;
/// // let message = CreateInteractionResponseMessage::new().embed(embed);
/// // let builder = CreateInteractionResponse::Message(message);
/// // interaction.create_response(&ctx.http, builder).await;
///
/// /* ----- */
#[proc_macro]
pub fn create_response_msg(tokens: TokenStream) -> TokenStream {
    _create_response_msg(tokens.into()).into()
}
