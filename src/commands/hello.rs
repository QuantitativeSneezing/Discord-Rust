use serenity::{
    builder::CreateApplicationCommand,
    client::Context,
    model::application::interaction::{
        InteractionResponseType,
        application_command::ApplicationCommandInteraction,
    },
};
use rand::seq::SliceRandom;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let mut test_var = "HELLO!!!!!!!!!!";
    if let Err(why) = command.create_interaction_response(
        &ctx,
        |r| {
            r.kind(InteractionResponseType::ChannelMessageWithSource)
             .interaction_response_data(|m| m.content(test_var))
        }
    ).await {
        eprintln!("error: {}", why)
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("Ping!")
}
