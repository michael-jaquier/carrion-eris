use std::env;

use carrion_eris::database::surreal::SurrealDB;
use carrion_eris::{command, State};

use poise::serenity_prelude as serenity;

use std::time::Duration;

use tracing::{debug, instrument};

use carrion_eris::game_loop::Handler;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

#[instrument]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().expect("Failed to read .env file");
    let filter = EnvFilter::from_default_env();
    println!("Filter: {:?}", filter);

    // For me this is <> file:///Users/michael.jaquier/carrion-eris/ce.db
    let db = env::var("DATABASE_URL").expect("Expected a database url in the environment");
    SurrealDB::connect(&db).await?;
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().compact())
        .init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    tokio::spawn(async move {
        let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
        let intents =
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;
        let mut client = serenity::Client::builder(&token, intents)
            .event_handler(Handler::default())
            .await
            .expect("Error creating client");

        if let Err(why) = client.start().await {
            eprintln!("Client error: {:?}", why);
        }
    });

    let options = poise::FrameworkOptions {
        commands: vec![
            command::help(),
            command::create(),
            command::character_trait(),
            command::delete(),
            command::me(),
            command::skill(),
            command::battle(),
            command::items(),
            command::sell(),
            command::equip(),
            command::sum(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("~".into()),
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
            additional_prefixes: vec![
                poise::Prefix::Literal("eris"),
                poise::Prefix::Literal("eris,"),
            ],
            ..Default::default()
        },
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                debug!("Got an event in event handler: {:?}", event.name());
                Ok(())
            })
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .token(token)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(State {})
            })
        })
        .options(options)
        .intents(intents);

    framework.run().await.unwrap();
    Ok(())
}
