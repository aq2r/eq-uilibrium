use proc_macro2::TokenStream;

pub fn _send_msg(tokens: TokenStream) -> TokenStream {
    todo!()
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
}
