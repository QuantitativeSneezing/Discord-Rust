use rand::{
    // Rng,
    // seq::SliceRandom,
    rngs::StdRng
};
// use std::i8::rem_euclid
use serenity::{
    builder::CreateApplicationCommand,
    client::Context,
    model::application::interaction::{
        InteractionResponseType,
        application_command::ApplicationCommandInteraction,
    },
};

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let possible_greetings = vec!["HELLO!!!!!!!!!!", "WASSUP", "HI", "GREETINGS", "HAI DOMO"];
    let mut random = rand::random::<i8>();
    if random < -1 {
        random = random * -1;
    }
    random= random % 5;
    let message= possible_greetings[random as usize];
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
