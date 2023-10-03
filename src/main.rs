/*
 *  note to self, use rustup for rust version management
 */

use std::collections::HashSet;

use dotenvy::dotenv_iter;
use serenity::{
    client::Client,
    model::gateway::GatewayIntents,
    http::Http,
};
use songbird::{
    SerenityInit,
    Config,
    driver::DecodeMode,
};

mod commands;
mod handler;
mod receiver;
mod stt_model;

use crate::{
    handler::*,
    stt_model::RECOGNIZER,
};

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    let mut token = String::new();
    for item in dotenv_iter().expect("error: opening .env") {
        let (k, v) = item.expect("error: getting token");
        if k == "TOKEN" {
            token = v;
        }
    }
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
                | GatewayIntents::GUILD_VOICE_STATES
                | GatewayIntents::MESSAGE_CONTENT
                ;
    {
        let mut recognizer = RECOGNIZER.lock().expect("error: acquiring voice recognizer lock");
        recognizer.set_max_alternatives(10);
        recognizer.set_words(true);
        recognizer.set_partial_words(true);
    }
    // decode all incoming voice packets
    let songbird_config = Config::default()
        .decode_mode(DecodeMode::Decode);
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .register_songbird_from_config(songbird_config)
        .await
        .expect("error");

    // Start listening for events by starting a single shard
    if let Err(e) = client.start().await {
        eprintln!("error: {:?}", e);
        println!("creating client with less privileges"); // Slash commands only
        Client::builder(
            &token,
            GatewayIntents::non_privileged()
            | GatewayIntents::GUILD_PRESENCES
            | GatewayIntents::GUILD_MEMBERS
            | GatewayIntents::GUILD_VOICE_STATES
        )
        .event_handler(Handler)
        .register_songbird_from_config(
            Config::default().decode_mode(DecodeMode::Decode)
        )
        .await
        .expect("error")
        .start()
        .await
        .expect("error")
    }
}
