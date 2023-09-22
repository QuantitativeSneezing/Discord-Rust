use std::collections::HashSet;

use serenity::{
    async_trait,
    client::Client,
    model::{
        channel::Message,
        gateway::{
            Ready,
            GatewayIntents,
        },
    },
    http::Http,
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let token = String::new();
    let http = Http::new(&token);
    let (_owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };
    let intents = GatewayIntents::non_privileged()
                                | GatewayIntents::GUILD_PRESENCES
                                | GatewayIntents::GUILD_MEMBERS
                                | GatewayIntents::MESSAGE_CONTENT
                                ;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("### Error creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        eprintln!("### Client error: {:?}", why);
        println!("### Attempting to create client without message content privilege (only slash commands will be available)");
        Client::builder(
            &token,
            GatewayIntents::non_privileged()
            | GatewayIntents::GUILD_PRESENCES
            | GatewayIntents::GUILD_MEMBERS
        )
            .event_handler(Handler)
            .await
            .expect("### Error creating lesser privileged client")
            .start()
            .await
            .expect("### Error starting lesser privileged client")
    }
}