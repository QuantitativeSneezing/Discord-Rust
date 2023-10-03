use serenity::{
    builder::CreateApplicationCommand,
    client::Context,
    model::{
        application::interaction::{
            InteractionResponseType,
            application_command::ApplicationCommandInteraction,
        },
        prelude::{
            application_command::CommandDataOptionValue,
            command::CommandOptionType,
        },
    },
};
use songbird::{
    self,
    CoreEvent,
};

use crate::receiver::Receiver;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    if let Some(lang) = command.data.options.get(0) {
        if let CommandDataOptionValue::String(lang) = lang.resolved.as_ref().expect("error: getting voice recognition language") {
            println!("voice recognition language: {lang}")
        }
    }

    let guild_id = command.guild_id.expect("error: getting guild ID");

    let manager = songbird::get(ctx)
        .await
        .expect("error: getting voice manager")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        handler.add_global_event(
            CoreEvent::SpeakingStateUpdate.into(),
            Receiver::new(),
        );

        handler.add_global_event(
            CoreEvent::SpeakingUpdate.into(),
            Receiver::new(),
        );

        handler.add_global_event(
            CoreEvent::VoicePacket.into(),
            Receiver::new(),
        );

        handler.add_global_event(
            CoreEvent::RtcpPacket.into(),
            Receiver::new(),
        );

        handler.add_global_event(
            CoreEvent::ClientDisconnect.into(),
            Receiver::new(),
        );

        if let Err(e) = command.create_interaction_response(
            &ctx,
            |r| {
                r.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|m| m.content("enabling voice recognition (Speech-To-Text)"))
            }
        ).await {
            eprintln!("error: {}", e)
        }
    } else {
        if let Err(e) = command.create_interaction_response(
            &ctx,
            |r| {
                r.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|m| m.content("error: enabling voice recognition"))
            }
        ).await {
            eprintln!("error: {}", e)
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("stt")
        .description("Voice recognition/Speech-To-Text")
        .create_option(
            |opt|
                opt
                    .name("language")
                    .description("Speech-To-Text voice recognition language")
                    .kind(CommandOptionType::String)
                    .required(false)
        )
}
