// use rand::{
//     Rng,
//     thread_rng,
//     seq::SliceRandom,

// };
use serenity::{
    builder::CreateApplicationCommand,
    client::Context,
    model::application::interaction::{
        InteractionResponseType,
        application_command::ApplicationCommandInteraction,
    },
};

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    // let mut rng = thread_rng();
    let test_var = vec!["HELLO!!!!!!!!!!", "WASSUP", "HI", "GREETINGS", "HAI DOMO"];
    // let mut rng = rand::thread_rng();
    // rng.gen::<i64>()
    // let message= test_var[0];
    if let Err(e) = command.create_interaction_response(
        &ctx,
        |r| {
            r.kind(InteractionResponseType::ChannelMessageWithSource)
             .interaction_response_data(|m| m.content(message))
        }
    ).await {
        eprintln!("error: {}", e)
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("hello").description("Hello")
}
