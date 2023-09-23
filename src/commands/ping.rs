use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::{
    InteractionResponseType,
    application_command::ApplicationCommandInteraction,
};

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    if let Err(why) = command.create_interaction_response(
        &ctx,
        |r| {
            r.kind(InteractionResponseType::ChannelMessageWithSource)
             .interaction_response_data(|m| m.content("pong"))
        }
    ).await {
        eprintln!("error: {}", why)
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("Ping!")
}
