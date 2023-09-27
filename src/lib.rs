pub mod battle;
pub mod classes;
pub mod commands;
pub mod database;
pub mod dice;
pub mod enemies;
pub mod items;
pub mod mutators;
pub mod player;
pub mod skills;
pub mod traits;
pub mod units;

// Custom user data passed to all command functions
pub struct State {}
type CarrionResult<T> = Result<T, CarrionError>;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, State, Error>;

use serde::{Deserialize, Serialize};

use surrealdb::sql::Thing;
use thiserror::Error;

use skills::Skill;

use crate::enemies::Enemy;
use crate::player::Character;

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
    pub action: Skill,
    pub enemy_action: String,
    pub damage_dealt: i32,
    pub damage_taken: i32,
    pub player_name: String,
    pub monster_name: String,
    pub kill: bool,
    pub critical: bool,
    pub leveled_up: bool,
    pub monster_hp: i32,
    pub traits_available: u32,
    pub next_level: u32,
    pub experience_gained: u32,
    pub skill_experience_gained: u32,
    pub gold_gained: u64,
}

impl BattleInfo {
    pub fn begin(character: &Character, enemy: &Enemy) -> Self {
        Self {
            action: character.current_skill.skill(),
            enemy_action: "".to_string(),
            damage_dealt: 0,
            damage_taken: 0,
            player_name: character.name.clone(),
            monster_name: enemy.kind.to_string(),
            kill: false,
            critical: false,
            leveled_up: false,
            monster_hp: enemy.health,
            traits_available: character.available_traits,
            next_level: character.experience_to_next_level(),
            experience_gained: 0,
            skill_experience_gained: 0,
            gold_gained: 0,
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
        string.push_str(&self.damage_dealt.to_string());
        string.push_str(" damage!");
        string.push_str("\t🎲");

        if self.damage_taken > 0 {
            string.push_str("\n\t");
            string.push_str("🎲\t");
            string.push_str(&self.monster_name);
            string.push_str(" attacked ");
            string.push_str(&self.player_name);
            string.push_str(" with ");
            string.push_str(&self.enemy_action.to_string());
            string.push_str(" dealing ");
            string.push_str(&self.damage_taken.to_string());
            string.push_str(" damage!");
            string.push_str("\t🎲");
        }

        if self.critical {
            string.push_str(" 💥 Critical hit! 💥");
        }
        if self.gold_gained > 0 {
            string.push_str("\n\t");
            string.push_str("💰\t");
            string.push_str(&self.player_name);
            string.push_str(" gained ");
            string.push_str(&self.gold_gained.to_string());
            string.push_str(" gold!");
            string.push_str("\t💰");
        }
        if self.leveled_up {
            string.push_str("\n\t");
            string.push_str("🎉\t");
            string.push_str("Leveled up!");
            string.push_str("\t🎉")
        }
        if self.traits_available > 0 {
            string.push_str("\n\t");
            string.push_str("⭐\t");
            string.push_str("Trait available!");
            string.push_str("\t⭐")
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
    let default_scale = |n: u32| ((n as f64).ln().powf(power.unwrap_or(1.1))) as u32;
    default_scale(n)
}

pub fn log_power_power_scale(n: u32) -> u32 {
    let default_scale = |n: u32| ((n as f64).powf(n as f64).ln() * n as f64) as u32;
    default_scale(n * n)
}

pub fn ln_power_power_power_scale(n: u32) -> u32 {
    let n = n as f64;
    let default_scale = |n: f64| 2.5_f64.powf(n + 3.0).ln() * (n + 10.0).powf(2.1);
    default_scale(n) as u32
}

trait ValidEnum {
    fn valid() -> String;
}

trait EnemyEvents {
    fn grade(&self) -> crate::enemies::MobGrade;
    fn actions(&self) -> Vec<crate::skills::MobAction>;

    fn alignment(&self) -> crate::units::Alignment;

    fn vulnerability(&self) -> Option<crate::units::DamageType>;
}
