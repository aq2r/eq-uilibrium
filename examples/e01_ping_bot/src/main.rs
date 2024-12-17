use serenity::{
    all::{
        Command, Context, CreateCommand, EventHandler, GatewayIntents, Interaction, Message, Ready,
    },
    async_trait, Client,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let ping_command = CreateCommand::new("ping").description("Pong!");
        match Command::create_global_command(&ctx.http, ping_command).await {
            Ok(_) => println!("Slash command has been registered."),
            Err(_) => println!("Failed to register slash command."),
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            let result =
                eq_uilibrium::send_msg!(&ctx.http, msg.channel_id, content = "Pong!").await;

            if let Err(err) = result {
                println!("Error sending message: {err:?}");
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match &interaction {
            Interaction::Command(inter) => {
                let command_name = inter.data.name.as_str();

                if command_name == "ping" {
                    let result =
                        eq_uilibrium::create_response_msg!(&ctx.http, inter, content = "Pong!")
                            .await;

                    if let Err(err) = result {
                        println!("Error interaction responce: {err:?}");
                    }
                }
            }

            _ => (),
        }
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN")
        .expect("The environment variable 'DISCORD_TOKEN' was not found.");

    let intents = GatewayIntents::all();

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(err) = client.start().await {
        println!("Client error: {err:?}");
    }
}
