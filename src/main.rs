/*
 *  note to self, use rustup for rust version management
 */

use std::collections::HashSet;

use serenity::{
    client::Client,
    model::gateway::GatewayIntents,
    http::Http,
};

mod commands;
mod handler;

use crate::handler::*;

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
        Err(why) => panic!("error: {:?}", why),
    };
    let intents = GatewayIntents::non_privileged()
                | GatewayIntents::GUILD_PRESENCES
                | GatewayIntents::GUILD_MEMBERS
                | GatewayIntents::MESSAGE_CONTENT
    ;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("error");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        eprintln!("error: {:?}", why);
        println!("creating client with less privileges"); // Slash commands only
        Client::builder(
            &token,
            GatewayIntents::non_privileged()
            | GatewayIntents::GUILD_PRESENCES
            | GatewayIntents::GUILD_MEMBERS
            | GatewayIntents::GUILD_VOICE_STATES
        )
        .event_handler(Handler)
        .await
        .expect("error")
        .start()
        .await
        .expect("error")
    }
}
