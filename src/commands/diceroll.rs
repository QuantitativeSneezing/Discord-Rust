use serenity::{
    builder::CreateApplicationCommand,
    client::Context,
    model::application::interaction::{
        InteractionResponseType,
        application_command::ApplicationCommandInteraction,
    },
};
pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {

}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("diceroll").description("Roll the dice!")
}
