use std::collections::HashSet;
use std::env;

use carrion_eris::database::surreal::consumer::SurrealConsumer;
use carrion_eris::database::surreal::producer::SurrealProducer;
use carrion_eris::database::surreal::SurrealDB;
use carrion_eris::{commands, CarrionError, State};

use poise::{async_trait, serenity_prelude as serenity};
use serenity::builder::CreateMessage;
use serenity::client::Context;
use serenity::http::CacheHttp;
use serenity::model::channel::{PrivateChannel, ReactionType};
use serenity::model::gateway::Ready;
use std::sync::atomic::AtomicBool;
use std::{collections::HashMap, env::var, sync::Mutex, time::Duration};

use serenity::model::id::{ChannelId, GuildId};
use serenity::model::prelude::component::ButtonStyle;
use tokio::time::sleep;

use tracing::{debug, error, info};
use tracing_subscriber;

use carrion_eris::battle::{all_battle, all_notify};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

struct Handler {
    is_loop_running: AtomicBool,
}

#[async_trait]
impl serenity::EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        let ctx_clone = ctx.clone();
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(10)).await;
                let results = all_battle().await;
                let channel_id = 1152198475925176401;

                let mut menu = String::from("```\n");
                if results.results.len() == 0 {
                    continue;
                }
                menu.push_str("Battle Results:\n");
                for battle in results.results.iter() {
                    menu.push_str(&format!("{}\n", battle));
                }
                menu += "\n```";
                let m = ChannelId(channel_id)
                    .send_message(&ctx_clone.http, |m| m.content(menu))
                    .await;
                if let Err(why) = m {
                    eprintln!("Error sending message: {:?}", why);
                };
                let notification_channel = 1152604267933339729;
                let notify = all_notify().await;
                if notify.len() == 0 {
                    continue;
                }
                let m = ChannelId(notification_channel)
                    .send_message(&ctx_clone.http, |m| {
                        m.content(format!("{}", notify.join("\n")))
                    })
                    .await;
                if let Err(why) = m {
                    eprintln!("Error sending message: {:?}", why);
                };
            }
        });
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().expect("Failed to read .env file");
    let filter = EnvFilter::from_default_env();
    SurrealDB::connect("http://localhost:8000").await?;
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    tokio::spawn(async move {
        let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
        let intents =
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;
        let mut client = serenity::Client::builder(&token, intents)
            .event_handler(Handler {
                is_loop_running: AtomicBool::new(false),
            })
            .await
            .expect("Error creating client");

        if let Err(why) = client.start().await {
            eprintln!("Client error: {:?}", why);
        }
    });

    let options = poise::FrameworkOptions {
        commands: vec![
            commands::help(),
            commands::create(),
            commands::character_trait(),
            commands::delete_character(),
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
