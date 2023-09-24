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

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::ytdl("").await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                if let Err(e) = command.create_interaction_response(
                    &ctx,
                    |r| {
                        r.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|m| m.content("error: ffmpeg"))
                    }
                ).await {
                    eprintln!("error: {}", e)
                }

                return ();
            },
        };

        handler.play_source(source);

        if let Err(e) = command.create_interaction_response(
            &ctx,
            |r| {
                r.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|m| m.content("playing in voice channel"))
            }
        ).await {
            eprintln!("error: {}", e)
        }
    } else {
        if let Err(e) = command.create_interaction_response(
            &ctx,
            |r| {
                r.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|m| m.content("error: playing in voice channel"))
            }
        ).await {
            eprintln!("error: {}", e)
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("playvc").description("Play in VC")
}
