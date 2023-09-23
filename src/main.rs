/*
 *  note to self, use rustup for rust version management
 */

use std::collections::HashSet;

use serenity::{
    async_trait,
    client::{
        Client,
        Context,
    },
    model::{
        application::{
            command::Command,
            interaction::{
                Interaction,
                InteractionResponseType,
            },
        },
        channel::Message,
        gateway::{
            Activity,
            Ready,
            GatewayIntents,
        },
    },
    http::Http,
    prelude::*,
};

mod commands;
use crate::commands::*;

struct Handler;

impl Handler {
    pub async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("{:?}", command);

            match command.data.name.as_str() {
                "ping" => ping::run(&ctx, &command).await,
                // ""     =>     ::run(&ctx, &command).await,

                _ => if let Err(why) = command.create_interaction_response(
                    &ctx.http,
                    |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content("error!"))
                    }
                ).await {
                    eprintln!("error: {}", why)
                }
            }
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("connected as {}", ready.user.name);

        let global_commands = Command::set_global_application_commands(
            &ctx.http,
            |commands| {
                commands.create_application_command(ping::register)
                        // .create_application_command(    ::register)
        }).await;
        // Command::delete_global_application_command(&ctx.http, serenity::model::id::CommandId()).await.expect("error");

        println!("available commands\n{:#?}", Vec::from_iter(global_commands.unwrap().iter().map(|c| &(c.name)))); // Available global slash commands

        ctx.set_activity(Activity::playing("being a bot")).await
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        self.interaction_create(ctx, interaction).await
    }
}

#[tokio::main]
async fn main() {
    let token = String::new();
    let http = Http::new(&token);
    let (_owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("error: {:?}", why),
    };
    let intents = GatewayIntents::non_privileged()
                | GatewayIntents::GUILD_PRESENCES
                | GatewayIntents::GUILD_MEMBERS
                | GatewayIntents::MESSAGE_CONTENT
    ;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("error");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        eprintln!("error: {:?}", why);
        println!("creating client with less privileges"); // Slash commands only
        Client::builder(
            &token,
            GatewayIntents::non_privileged()
            | GatewayIntents::GUILD_PRESENCES
            | GatewayIntents::GUILD_MEMBERS
        )
        .event_handler(Handler)
        .await
        .expect("error")
        .start()
        .await
        .expect("error")
    }
}
