use std::env;
use dotenv::dotenv;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const HELLO_MESSAGE: &str = "
Greetings Human!

You have requested my presense!

I am a bot created to make Fortnut a great server with many features.

? Want to request a feature ?
=> Ask ParaDucks if it can be done.

? Something broken ?
=> Reach out to me to get it fixed.

I hope this was a good introduction!
-- FortBot
";

const HELLO_COMMAND: &str = "!hello";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg:Message) {
        if msg.content == HELLO_COMMAND{
            if let Err(why) = msg.channel_id.say(&ctx.http, HELLO_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready (&self, _: Context, ready: Ready) {
        println!("{} has connected", ready.user.name);
    }
}

#[tokio::main]
async fn main() {

    dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
