use serenity::{
    async_trait,
    client::Context,
    model::{
        application::{
            command::Command,
            interaction::{
                Interaction,
                InteractionResponseType,
            },
        },
        gateway::{
            Activity,
            Ready,
        },
    },
    prelude::*,
};

use crate::commands::*;

pub struct Handler;

impl Handler {
    pub async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("{:?}", command);

            match command.data.name.as_str() {
                "ping"    =>    ping::run(&ctx, &command).await,
                "joinvc"  =>  joinvc::run(&ctx, &command).await,
                "leavevc" => leavevc::run(&ctx, &command).await,
                "playvc"  =>  playvc::run(&ctx, &command).await,
                "hello"   =>   hello::run(&ctx, &command).await,

                _ => if let Err(e) = command.create_interaction_response(
                    &ctx.http,
                    |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content("error!"))
                    }
                ).await {
                    eprintln!("error: {}", e)
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
                commands.create_application_command(   ping::register)
                        .create_application_command( joinvc::register)
                        .create_application_command(leavevc::register)
                        .create_application_command( playvc::register)
        }).await;
        // Command::delete_global_application_command(&ctx.http, serenity::model::id::CommandId()).await.expect("error");

        println!("available commands\n{:#?}", Vec::from_iter(global_commands.unwrap().iter().map(|c| &(c.name)))); // Available global slash commands

        ctx.set_activity(Activity::playing("being a bot")).await
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        self.interaction_create(ctx, interaction).await
    }
}
