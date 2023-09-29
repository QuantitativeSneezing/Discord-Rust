use serenity::{
    builder::CreateApplicationCommand,
    client::Context,
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        prelude::Channel,
    },
};

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let author = command.member.clone().expect("RIP NO AUTHOR LOL");
    let author_name = author.display_name();
    // let author_id = author.user.id;
    // let cache_http= ctx.http;
    let dm = author
        .user
        .direct_message(&ctx, |m| m.content(format!("hello, \"{}\"", author_name)))
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
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("dm").description("DM")
}
