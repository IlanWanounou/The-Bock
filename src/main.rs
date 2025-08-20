use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use dotenv::dotenv;

struct Handler;

use songbird::SerenityInit;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!join" {
            if let Some(guild_id) = msg.guild_id {
                match ctx.http.get_guild(guild_id).await {
                    Ok(guild) => {
                        match guild.channels(&ctx.http).await {
                            Ok(channels) => {
                                if let Some((_, channel)) = channels.iter()
                                    .find(|(_, c)| c.kind == serenity::model::channel::ChannelType::Voice)
                                {
                                    let manager = songbird::get(&ctx)
                                        .await
                                        .expect("Songbird Voice client not initialized");

                                    let join_result = manager.join(guild_id, channel.id).await;

                                    if let Ok(_success) = join_result {
                                        let _ = msg.channel_id.say(&ctx.http, format!("Successfully joined {}", channel.name)).await;
                                    } else {
                                        let _ = msg.channel_id.say(&ctx.http, "Failed to join voice channel").await;
                                    }
                                }
                            }
                            Err(err) => eprintln!("Erreur récupération des channels: {:?}", err),
                        }
                    },
                    Err(err) => {
                        eprintln!("Erreur récupération de la guilde: {:?}", err);
                        let _ = msg.channel_id.say(&ctx.http, "Erreur lors de la récupération des informations de la guilde").await;
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_VOICE_STATES | GatewayIntents::GUILDS;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler)
            .register_songbird()
            .await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}