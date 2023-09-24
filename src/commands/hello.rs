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
    let mut rng = rand::thread_rng();
    let mut test_var = vec!["HELLO!!!!!!!!!!", "WASSUP", "HI", "GREETINGS", "HAI DOMO"];
    test_var.shuffle(&mut rng);
    if let Err(why) = command.create_interaction_response(
        &ctx,
        |r| {
            r.kind(InteractionResponseType::ChannelMessageWithSource)
             .interaction_response_data(|m| m.content(test_var.first().unwrap()))
        }
    ).await {
        eprintln!("error: {}", why)
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("hello").description("Hello")
}
