use serenity::{
    // builder::CreateApplicationCommand,
    client::Context,
    model::prelude::Message,
    utils::MessageBuilder,
};

pub async fn run(ctx: Context, msg: Message) {
    if msg.content.len() > 0 {

        // let channel = match msg.channel_id.to_channel(&ctx).await {
        //     Ok(channel) => channel,
        //     Err(why) => {
        //         println!("Error getting channel: {:?}", why);

        //         return;
        //     }
        // };
        if let Ok(channel_info) = msg.channel(&ctx).await {
            match channel_info.guild() {
                Some(guild_channel) => {
                    println!("It's a guild channel named {}!", guild_channel.name);
                }
                None => {
                    let response = MessageBuilder::new()
                        .push(format!(r#"you said "{}" "#, msg.content))
                        .push("")
                        .build();

                    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
        } else {
            println!("channel info failed")
        }
    }
}

// pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
//     command.name("respondtodm").description("")
// }
