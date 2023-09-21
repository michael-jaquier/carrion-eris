pub mod battle;
pub mod classes;
pub mod commands;
pub mod database;
pub mod dice;
pub mod enemies;
pub mod items;
pub mod mutators;
pub mod player;
pub mod traits;
pub mod units;

// Custom user data passed to all command functions
pub struct State {}
type CarrionResult<T> = Result<T, CarrionError>;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, State, Error>;

use rand::Rng;
use serde::{Deserialize, Serialize};

use surrealdb::sql::Thing;
use thiserror::Error;

use crate::player::PlayerAction;

use crate::units::DamageType;
use std::fmt::{Display, Formatter};

#[derive(Error, Debug)]
pub enum CarrionError {
    #[error("Surreal error: {0}")]
    SurrealDBError(#[from] surrealdb::error::Db),
    #[error("Surreal error: {0}")]
    SurrealApiError(#[from] surrealdb::error::Api),
    #[error("Surreal error: {0}")]
    SurrealError(#[from] surrealdb::Error),
    #[error("Parse error: {0}")]
    ParseError(String),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleInfo {
    pub action: PlayerAction,
    pub damage: i32,
    pub player_name: String,
    pub monster_name: String,
    pub kill: bool,
    pub critical: bool,
    pub leveled_up: bool,
    pub monster_hp: i32,
    pub traits_available: u32,
    pub next_level: u32,
}

impl BattleInfo {
    pub fn new(
        action: PlayerAction,
        damage: i32,
        player_name: String,
        monster_name: String,
        kill: bool,
        critical: bool,
        leveled_up: bool,
        monster_hp: i32,
        traits_available: u32,
        next_level: u32,
    ) -> Self {
        Self {
            action,
            damage,
            player_name,
            monster_name,
            kill,
            critical,
            leveled_up,
            monster_hp,
            traits_available,
            next_level,
        }
    }
}

impl Display for BattleInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        string.push_str("\n🗡️");
        string.push_str("\n\t");
        string.push_str("🎲\t");
        string.push_str(&self.player_name);
        string.push_str(" attacked the ");
        string.push_str(&self.monster_name);
        string.push_str(" with ");
        string.push_str(&self.action.to_string());
        string.push_str(" dealing ");
        string.push_str(&self.damage.to_string());
        string.push_str(" damage!");
        string.push_str("\t🎲");
        if self.critical {
            string.push_str(" 💥 Critical hit! 💥");
        }
        if self.kill {
            string.push_str("\n\t");
            string.push_str("☠️\t");
            string.push_str("Killing blow");
            string.push_str("\t☠️");
        }
        if self.leveled_up {
            string.push_str("\n\t");
            string.push_str("🎉\t");
            string.push_str("Leveled up!");
            string.push_str("\t🎉")
        }
        if self.traits_available > 0 {
            string.push_str("\n\t");
            string.push_str("🎉\t");
            string.push_str("Trait available!");
            string.push_str("\t🎉")
        }
        string.push_str("\n🗡️\n");
        write!(f, "{}", string)
    }
}

trait AttributeScaling {
    fn scaling(&self) -> Option<crate::units::Attribute>;
}

trait ElementalScaling {
    fn scaling(&self) -> Option<crate::units::DamageType>;
}

pub fn log_power_scale(n: u32, power: Option<f64>) -> u32 {
    let default_scale = |n: u32| ((n as f64).ln().powf(power.unwrap_or(1.1))).floor() as u32;
    default_scale(n)
}
