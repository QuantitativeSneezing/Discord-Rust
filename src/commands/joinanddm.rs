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
    let author = command.member.clone().expect("RIP NO AUTHOR LOL");
    let author_name = author.display_name();
    // println!(author_name)
    // let channel_id = command.channel_id;
    let channels = guild.channels(ctx).await.expect("error: getting channels");
    let voice_channel = channels
        .values()
        .find(|ch| ch.kind == Voice)
        .expect("error: getting voice channel");
    let voice_channel_id = voice_channel.id;
    let dm = author
    .user
    .direct_message(&ctx, |m| m.content(format!("Hello, {}, I love you", author_name)))
    .await;
    match dm {
        Ok(_) => {
            if let Err(e) = command
                .create_interaction_response(&ctx, |r| {
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|m| m.content("dm sent"))
                })
                .await
            {
                eprintln!("error: {}", e)
            }
        }
        Err(why) => {
            println!("Err sending help: {:?}", why);
        }
    }
    let manager = songbird::get(ctx)
        .await
        .expect("error: getting voice manager")
        .clone();

    let (handler_lock, conn_result) = manager.join(guild_id, voice_channel_id).await;

    match conn_result {
        Ok(_) => {
            let mut _handler = handler_lock.lock().await;

            if let Err(e) = command.create_interaction_response(
                &ctx,
                |r| {
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|m| m.content("joined voice channel"))
                }
            ).await {
                eprintln!("error: {}", e)
            }
        },

        Err(_) =>
            if let Err(e) = command.create_interaction_response(
                &ctx,
                |r| {
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|m| m.content("error: joining voice channel"))
                }
            ).await {
                eprintln!("error: {}", e)
            },
    }

}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("joinanddm").description("Joins and DMs you")
}
