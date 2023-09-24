use serenity::{
    builder::CreateApplicationCommand,
    client::Context,
    model::{
        application::interaction::{
            InteractionResponseType,
            application_command::ApplicationCommandInteraction,
        },
    },
};
use songbird;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let guild_id = command.guild_id.expect("error: getting guild ID");

    let manager = songbird::get(ctx)
        .await
        .expect("error: getting voice manager")
        .clone();

    match manager.leave(guild_id).await {
        Ok(_) =>
            if let Err(e) = command.create_interaction_response(
                &ctx,
                |r| {
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|m| m.content("left voice channel"))
                }
            ).await {
                eprintln!("error: {}", e)
            },
        Err(_) =>
            if let Err(e) = command.create_interaction_response(
                &ctx,
                |r| {
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|m| m.content("error: leaving voice channel"))
                }
            ).await {
                eprintln!("error: {}", e)
            },
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("leavevc").description("Leave VC")
}
