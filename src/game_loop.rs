use poise::async_trait;
use serenity::client::Context;
use serenity::model::channel::GuildChannel;
use serenity::model::gateway::Presence;
use serenity::model::id::{ChannelId, GuildId};
use std::sync::Arc;
use tokio::sync::RwLock;

use lazy_static::lazy_static;
use std::time::Duration;

use crate::database::surreal::SurrealDB;
use crate::game::buffer::Buffer;
use crate::game::data::GameData;
use tokio::time::sleep;
use tracing::info;
use tracing::trace;
lazy_static! {
    pub static ref BUFFER: Arc<RwLock<Buffer>> = Arc::new(RwLock::new(Buffer::new()));
    pub static ref GAME: Arc<RwLock<GameData>> = Arc::new(RwLock::new(GameData::new()));
}

#[derive(Debug, Clone, Default)]
pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl serenity::client::EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        info!("Cache Ready");
        let ctx_clone = ctx.clone();
        tokio::spawn(async move {
            {
                let mut game_data = GAME.write().await;
                game_data.init().await;
            }

            loop {
                trace!("Starting Battle Loop");

                let sleep_duration = Duration::from_secs(6);
                let game_data = GAME.read().await;
                let results = game_data.battle().await;
                drop(game_data);
                // Spawn a new asynchronous task to run Buffer::mutations().await repeatedly
                let mutations_task = tokio::spawn(async {
                    loop {
                        Buffer::mutations().await;
                        sleep(Duration::from_millis(25)).await; // Wait 50ms between mutations
                    }
                });

                if results.result.is_empty() {
                    sleep(Duration::from_secs(6)).await;
                    continue;
                }

                let channel_id = 1152198475925176401;

                let mut menu = String::from("```\n");

                if results.result.is_empty() {
                    sleep(Duration::from_secs(6)).await;
                    continue;
                }
                menu.push_str("Battle Results:\n");
                for battle in results.result.iter() {
                    menu.push_str(&format!("{}\n", battle));
                }
                menu += "\n```";
                let m = ChannelId(channel_id)
                    .send_message(&ctx_clone.http, |m| m.content(menu))
                    .await;
                if let Err(why) = m {
                    eprintln!("Error sending message: {:?}", why);
                };

                let synchro_db = GAME.read().await;
                synchro_db.synchronize_db().await;
                SurrealDB::export("test.db").await;
                sleep(sleep_duration).await;
                mutations_task.abort();
            }
        });
    }

    async fn channel_create(&self, _ctx: Context, channel: &GuildChannel) {
        info!("channel_create");
        info!("Channel: {:?}", channel);
    }

    async fn presence_update(&self, _ctx: Context, new_data: Presence) {
        info!("Presence Update: {:?}", new_data);
    }
}
