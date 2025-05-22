mod commands;

use std::env;
use::dotenv::dotenv;

use sqlx::mysql::MySqlPoolOptions;
use sqlx::pool::Pool;
use sqlx::MySql;

use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
use serenity::model::id::{GuildId, ChannelId};
use serenity::model::guild::Member;
use serenity::prelude::*;

struct Handler {
    database: Pool<MySql>,
}



#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, _member: Member) {

        if let Err(why) = ChannelId::new(1371059022245138546).say(&ctx.http, "Hello!").await {
            println!("Error sending message: {why:?}");
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an authentication error, or lack
            // of permissions to post in the channel, so log to stdout when some error happens,
            // with a description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {

            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                "id" => Some(commands::id::run(&command.data.options())),
                "attachmentinput" => Some(commands::attachmentinput::run(&command.data.options())),
                "updatemoney" => Some(commands::database::updatemoney::run(&command.data.options(), &self.database).await),
                "adduser" => Some(commands::database::adduser::run(&command.data.options(), &self.database).await),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = guild_id
            .set_commands(&ctx.http, vec![
                commands::ping::register(),
                commands::id::register(),
                commands::welcome::register(),
                commands::numberinput::register(),
                commands::attachmentinput::register(),
                commands::database::updatemoney::register(),
                commands::database::adduser::register(),
            ])
            .await;

        println!("I now have the following guild slash commands: {commands:#?}");

        let guild_command =
            Command::create_global_command(&ctx.http, commands::wonderful_command::register())
                .await;

        println!("I created the following global slash command: {guild_command:#?}");
    }
}

#[tokio::main(flavor="current_thread")]
async fn main() {
    dotenv().ok();
    env_logger::init();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES |
                                  GatewayIntents::DIRECT_MESSAGES |
                                  GatewayIntents::MESSAGE_CONTENT |
                                  GatewayIntents::GUILD_MEMBERS;

    let database: Pool<MySql> = MySqlPoolOptions::new()
            .max_connections(5)
            .connect("mariadb://root:root@localhost/discord-bot").await.expect("Unable to connect to DB");

    // Build our client.
    let mut client = Client::builder(token, intents)
        .event_handler(Handler{database})
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
