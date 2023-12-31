use serenity::{
    builder::CreateApplicationCommand,
    client::Context,
    model::{
        application::interaction::{
            InteractionResponseType,
            application_command::ApplicationCommandInteraction,
        },
        prelude::{
            application_command::{
                CommandDataOption,
                CommandDataOptionValue,
            },
            command::CommandOptionType,
        },
    },
};
use songbird;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let cmd_dat_opts: &[CommandDataOption] = &command.data.options;
    let cmd_dat_val = cmd_dat_opts.get(0).expect("error: getting URL").resolved.as_ref().expect("error: getting URL");
    let url = match cmd_dat_val {
        CommandDataOptionValue::String(url) => url,
        _ => return (),
    };

    let guild_id = command.guild_id.expect("error: getting guild ID");

    let manager = songbird::get(ctx)
        .await
        .expect("error: getting voice manager")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::ytdl(&url).await {
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
    command
        .name("playvc")
        .description("Play in VC")
        .create_option(
            |opt|
                opt
                    .name("url")
                    .description("Downloadable URL")
                    .kind(CommandOptionType::String)
                    .required(true)
        )
}
