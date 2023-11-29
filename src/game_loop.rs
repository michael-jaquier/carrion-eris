use poise::async_trait;
use poise::serenity_prelude::{Message, PrivateChannel};
use serenity::client::Context;
use serenity::model::channel::GuildChannel;
use serenity::model::gateway::Presence;
use serenity::model::id::GuildId;

use tokio::sync::OnceCell;

use std::time::Duration;

use crate::battle::BattleResult;
use crate::character::Character;
use crate::class::Classes;
use crate::database::Database;
use crate::enemy::Mob;
use crate::game::buffer::Buffer;
use crate::game::data::GameData;
use crate::game::mutations::Mutations;
use crate::skill::Skill;
use tokio::time::sleep;
use tracing::trace;
use tracing::{error, info, instrument, span, Level};

static GAME: OnceCell<GameData> = OnceCell::const_new();

pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(3);

#[derive(Debug, Clone, Default)]
pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        Self {}
    }
}

async fn set_game_mode(database: Database) {
    GAME.get_or_init(|| async {
        let mut game = GameData::new(database);
        game.init().await;
        game
    })
    .await;
}

pub async fn get_game() -> &'static GameData {
    GAME.get().unwrap_or_else(|| panic!("Game not initialized"))
}

pub async fn get_buffer() -> &'static Buffer {
    get_game().await.get_buffer()
}

#[instrument(skip(tx), name = "Serenity Loop")]
async fn serenity_loop(tx: tokio::sync::mpsc::Sender<String>, character_id: u64) {
    info!("Starting Serenity Loop");
    tokio::spawn(async move {
        loop {
            trace!("Starting Battle Loop");
            let mutations_task = tokio::spawn(async move {
                loop {
                    get_game().await.apply_mutations(character_id).await;
                    sleep(Duration::from_micros(125)).await;
                }
            });
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

            sleep(HEARTBEAT_INTERVAL).await;
            mutations_task.abort();
            drop(mutations_task)
        }
    });
}

async fn sync_db() {
    let instant = tokio::time::Instant::now();
    get_game().await.synchronize_db().await;
    span!(Level::TRACE, "Database", elapsed=?instant.elapsed());
}

#[async_trait]
impl serenity::client::EventHandler for Handler {
    async fn cache_ready(&self, _ctx: Context, _guilds: Vec<GuildId>) {
        info!("Cache Ready");
        set_game_mode(Database::Surreal).await;
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

pub async fn battle_info(character_id: u64) -> BattleResult {
    let game = get_game().await;
    if game.get_character(123).is_none() {
        let name = "a";
        let id = 123;
        let class = Classes::Warrior;
        let new_character = Character::new(name.to_string(), id, class);
        get_buffer()
            .await
            .add(Mutations::Create(Box::new(new_character)));
    }
    let results = get_game().await.battle(character_id).await;

    results
}

pub struct SynchroBattle {
    game: GameData,
}

impl SynchroBattle {
    pub async fn init() -> SynchroBattle {
        let mut game = GameData::new(Database::Mock);
        game.init().await;
        Self { game }
    }

    pub fn create_character(&self, name: String, id: u64, class: Classes) {
        let new_character = Character::new(name, id, class);
        self.game
            .get_buffer()
            .add(Mutations::Create(Box::new(new_character)));
    }

    pub fn delete_character(&self, id: u64) {
        self.game.get_buffer().add(Mutations::Delete(id));
    }

    pub fn change_skill(&self, id: u64, skill: Skill) {
        self.game.get_buffer().add(Mutations::Skill(id, skill));
    }

    pub fn add_enemy(&self, id: u64, mob: Mob, count: u32) {
        self.game
            .get_buffer()
            .add(Mutations::AddEnemy(id, mob, count));
    }
}
