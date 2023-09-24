use serenity::{
    builder::CreateApplicationCommand,
    client::Context,
    model::{
        application::interaction::{
            InteractionResponseType,
            application_command::ApplicationCommandInteraction,
        },
        prelude::ChannelType::Voice,
    },
};
use songbird;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let guild_id = command.guild_id.expect("error: getting guild ID");
    let guild = ctx.http.get_guild(*guild_id.as_u64()).await.expect("error: getting guild");

    // let channel_id = command.channel_id;
    let channels = guild.channels(ctx).await.expect("error: getting channels");
    let voice_channel = channels
        .values()
        .find(|ch| ch.kind == Voice)
        .expect("error: getting voice channel");
    let voice_channel_id = voice_channel.id;

    let manager = songbird::get(ctx)
        .await
        .expect("error: getting voice manager")
        .clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            if let Err(why) = command.create_interaction_response(
                &ctx,
                |r| {
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                     .interaction_response_data(|m| m.content("error: not in a voice channel"))
                }
            ).await {
                eprintln!("error: {}", why)
            }

            return ();
        },
    };

    let mut handler = handler_lock.lock().await;

    if let Err(e) = handler.join(voice_channel_id).await {
        if let Err(why) = command.create_interaction_response(
            &ctx,
            |r| {
                r.kind(InteractionResponseType::ChannelMessageWithSource)
                 .interaction_response_data(|m| m.content("error: not in a voice channel"))
            }
        ).await {
            eprintln!("error: {}", why)
        }
    } else {
        if let Err(why) = command.create_interaction_response(
            &ctx,
            |r| {
                r.kind(InteractionResponseType::ChannelMessageWithSource)
                 .interaction_response_data(|m| m.content("error: joining voice channel"))
            }
        ).await {
            eprintln!("error: {}", why)
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("joinvc").description("Join VC")
}
