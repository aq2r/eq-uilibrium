# eq-uilibrium

`serenity` の `send_message` と `create_responce(Message)` を Python のキーワード引数のように使用できるマクロ

Macros that allow `serenity`'s `send_message` and `create_responce(Message)` to be used like Python keyword arguments

## Examples

#### send_message

```rust
/* ----- */

let embed = CreateEmbed::new().title("embed").description("description");

eq_uilibrium::send_msg!(&http, channel_id, embed = embed, tts = true).await;
// let builder = CreateMessage::new().content("content").tts(true).embed(embed);
// let _ = channel_id.send_message(&http, builder).await;

/* ----- */
```

#### create_response_msg
```rust
/* ----- */

let embed = CreateEmbed::new().title("embed").description("description");

eq_uilibrium::create_response_msg!(&http, interaction, embed = embed, tts = true).await;
// let message = CreateInteractionResponseMessage::new().embed(embed);
// let builder = CreateInteractionResponse::Message(message);
// interaction.create_response(&ctx.http, builder).await?;

/* ----- */
```

#### Lisense

<sub>
Licensed under either of <a href="LICENCE-APACHE">Apache License, Version2.0</a> or
<a href="LICENCE-MIT">MIT Lisence</a> at your option.
</sub>
<br>
<sub>
Copyright (c) 2024 aq2r
</sub>