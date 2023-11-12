use poise::async_trait;
use poise::serenity_prelude::{Message, PrivateChannel};
use serenity::client::Context;
use serenity::model::channel::GuildChannel;
use serenity::model::gateway::Presence;
use serenity::model::id::GuildId;

use tokio::sync::OnceCell;

use std::time::Duration;

use crate::game::buffer::Buffer;
use crate::game::data::GameData;
use tokio::time::sleep;
use tracing::trace;
use tracing::{error, info};

static GAME: OnceCell<GameData> = OnceCell::const_new();
static BUFFER: OnceCell<Buffer> = OnceCell::const_new();

pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(3);

#[derive(Debug, Clone, Default)]
pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        Self {}
    }
}

pub async fn get_game() -> &'static GameData {
    GAME.get_or_init(|| async {
        let mut game = GameData::new();
        game.init().await;
        game
    })
    .await;
    GAME.get().unwrap()
}

pub async fn get_buffer() -> &'static Buffer {
    BUFFER.get_or_init(|| async { Buffer::new() }).await;
    BUFFER.get().unwrap()
}
async fn serenity_loop(tx: tokio::sync::mpsc::Sender<String>, character_id: u64) {
    tokio::spawn(async move {
        loop {
            trace!("Starting Battle Loop");
            let results = get_game().await.battle(character_id).await;

            if results.result.is_empty() {
                continue;
            }

            let mut battle_info = String::from("```\n");
            battle_info.push_str("Battle Results:\n");
            for battle in results.result.iter() {
                battle_info.push_str(&format!("{}\n", battle));
            }
            battle_info += "\n```";

            if let Err(e) = tx.send(battle_info.clone()).await {
                error!("Error sending message: {:?}", e);
            } // Spawn a new asynchronous task to run Buffer::mutations().await repeatedly
            let mutations_task = tokio::spawn(async move {
                loop {
                    Buffer::mutations(character_id).await;
                    sleep(Duration::from_micros(125)).await;
                }
            });
            sleep(HEARTBEAT_INTERVAL).await;
            mutations_task.abort();
            drop(mutations_task)
        }
    });
}

async fn sync_db() {
    let instant = tokio::time::Instant::now();
    get_game().await.synchronize_db().await;
    info!("Database Synchronized in {:?}", instant.elapsed());
}

#[async_trait]
impl serenity::client::EventHandler for Handler {
    async fn cache_ready(&self, _ctx: Context, _guilds: Vec<GuildId>) {
        info!("Cache Ready");

        tokio::spawn(async move {
            sync_db().await;
            sleep(HEARTBEAT_INTERVAL * 3).await;
        });
    }

    async fn channel_create(&self, _ctx: Context, channel: &GuildChannel) {
        info!("channel_create");
        info!("Channel: {:?}", channel);
    }

    async fn presence_update(&self, _ctx: Context, new_data: Presence) {
        info!("Presence Update: {:?}", new_data);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "eris start" {
            println!("Starting Eris");
            let dm = msg.author.create_dm_channel(&ctx.http).await;
            match dm {
                Ok(channel) => {
                    private_message(channel, ctx.clone()).await;
                }
                Err(e) => {
                    println!("Error creating DM channel: {:?}", e);
                }
            }
        }
    }
}

async fn private_message(channel: PrivateChannel, ctx: Context) {
    tokio::spawn(async move {
        let (tx, mut rx) = tokio::sync::mpsc::channel(32);
        let id = channel.recipient.id.as_u64();
        serenity_loop(tx.clone(), *id).await;

        tokio::spawn(async move {
            loop {
                sync_db().await;
                sleep(HEARTBEAT_INTERVAL * 3).await;
            }
        });

        while let Some(msg) = rx.recv().await {
            let m = channel.send_message(&ctx.http, |m| m.content(msg)).await;
            if let Err(why) = m {
                eprintln!("Error sending message: {:?}", why);
            };
        }
    });
}
