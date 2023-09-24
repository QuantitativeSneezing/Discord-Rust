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
    let mut test_var = vec!["HELLO!!!!!!!!!!", "WASSUP", "HI", "GREETINGS", "HAI DOMO"];
    // test_var.shuffle(&mut rng);
    if let Err(e) = command.create_interaction_response(
        &ctx,
        |r| {
            r.kind(InteractionResponseType::ChannelMessageWithSource)
             .interaction_response_data(|m| m.content(test_var.first().unwrap()))
        }
    ).await {
        eprintln!("error: {}", e)
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("hello").description("Hello")
}
