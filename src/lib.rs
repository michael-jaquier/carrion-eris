pub mod battle;
pub mod class;
pub mod command;
pub mod items;
#[rustfmt::skip]
pub mod constructed;
pub mod database;

pub mod character;
pub mod damage;
pub mod enemy;
mod game;
pub mod game_loop;
pub mod item;
pub mod skill;
pub mod r#trait;
pub mod unit;

// Custom user data passed to all command functions
#[derive(Debug)]
pub struct State {}
type CarrionResult<T> = Result<T, CarrionError>;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, State, Error>;

use item::IndividualItem;
use serde::{Deserialize, Serialize};
use std::f64::consts::E;
use tracing::{instrument, Level};

use surrealdb::sql::Thing;
use thiserror::Error;

use skill::Skill;

use crate::character::Character;
use crate::enemy::{Enemy, Mob};

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

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MobQueue {
    pub(crate) mobs: Vec<Mob>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleInfo {
    pub action: Skill,
    pub enemy_action: String,
    pub enemy_healing_action: String,
    pub player_damage: i32,
    pub enemy_damage: i32,
    pub player_healing: i32,
    pub enemy_healing: i32,
    pub player_name: String,
    pub monster_name: String,
    pub player_killed: bool,
    pub enemy_killed: bool,
    pub critical: bool,
    pub leveled_up: bool,
    pub monster_hp: i32,
    pub enemy_level: u32,
    pub traits_available: u32,
    pub next_level: u64,
    pub experience_gained: u64,
    pub skill_experience_gained: u64,
    pub gold_gained: u64,
    pub item_gained: Vec<IndividualItem>,
    pub number_of_player_attacks: i32,
    pub number_of_enemy_attacks: i32,
    pub custom_text: Option<String>,
}

impl BattleInfo {
    pub fn begin(character: &Character, enemy: &Enemy) -> Self {
        Self {
            action: character.current_skill.skill(),
            enemy_action: "".to_string(),
            player_damage: 0,
            enemy_damage: 0,
            player_healing: 0,
            enemy_healing: 0,
            player_name: character.name.clone(),
            monster_name: enemy.kind.to_string(),
            player_killed: false,
            enemy_killed: false,
            critical: false,
            leveled_up: false,
            monster_hp: enemy.health,
            traits_available: character.available_traits,
            next_level: character.experience_to_next_level(),
            experience_gained: 0,
            skill_experience_gained: 0,
            gold_gained: 0,
            item_gained: vec![],
            number_of_player_attacks: 0,
            number_of_enemy_attacks: 0,
            custom_text: None,
            enemy_level: enemy.level,
            enemy_healing_action: "".to_string(),
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
        if self.enemy_killed && self.player_damage == 0 {
            string.push_str("INSTANT DEATH !!");
        } else {
            string.push_str(&self.player_damage.to_string());
            string.push_str(" damage!");
        }
        string.push_str("\t🎲");
        string.push_str("\n\t");

        if self.number_of_player_attacks > 0 {
            string.push_str("🎲\t");
            string.push_str("Average damage per hit");
            string.push_str(" is ");
            string.push_str(
                &self
                    .player_damage
                    .saturating_div(self.number_of_player_attacks)
                    .to_string(),
            );
            string.push_str("\t🎲");
            string.push_str("\n\t")
        }

        if let Some(custom) = &self.custom_text {
            string.push_str("🎲\t");
            string.push_str(custom);
            string.push_str("\t🎲");
            string.push_str("\n\t");
        }

        if self.player_healing > 0 {
            string.push_str("🎲\t");
            string.push_str(&self.player_name);
            string.push_str(" healed ");
            string.push_str(&self.player_name);
            string.push_str(" with ");
            string.push_str(&self.action.to_string());
            string.push_str(" regenerating ");
            string.push_str(&self.player_healing.to_string());
            string.push_str(" health!");
            string.push_str("\t🎲");
            string.push_str("\n\t");
        }

        string.push_str("🪨\t");
        string.push_str("Next level in ");
        string.push_str(&self.next_level.to_string());
        string.push_str(" experience!");
        string.push_str("\t🪨");

        if self.enemy_damage > 0 {
            string.push_str("\n\t");
            string.push_str("🎲\t");
            string.push_str(&self.monster_name);
            string.push_str(" attacked ");
            string.push_str(&self.player_name);
            string.push_str(" with ");
            string.push_str(&self.enemy_action.to_string());
            string.push_str(" dealing ");
            string.push_str(&self.enemy_damage.to_string());
            string.push_str(" damage!");
            string.push_str("\t🎲");
        }

        if self.enemy_healing > 0 {
            string.push_str("\n\t");
            string.push_str("🎲\t");
            string.push_str(&self.monster_name);
            string.push_str(" healed ");
            string.push_str("with ");
            string.push_str(&self.enemy_healing_action.to_string());
            string.push_str(" regenerating ");
            string.push_str(&self.enemy_healing.to_string());
            string.push_str(" health!");
            string.push_str("\t🎲");
        }

        if self.experience_gained > 0 {
            string.push_str("\n\t");
            string.push_str("💠\t");
            string.push_str("Gained ");
            string.push_str(&self.experience_gained.to_string());
            string.push_str(" experience!");
            string.push_str("\t💠");
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

        if !self.item_gained.is_empty() {
            for item in &self.item_gained {
                string.push_str(format!("\n\t🎁\t Item gained: {}\t🎁", &item.name).as_str());
            }
        }
        string.push_str("\n🗡️\n");
        write!(f, "{}", string)
    }
}

trait AttributeScaling {
    fn scaling(&self) -> Option<String>;
}

trait ElementalScaling {
    fn scaling(&self) -> Option<damage::DamageType>;
}

pub fn log_power_scale(n: i32, power: Option<f64>) -> u32 {
    let default_scale = |n: i32| ((n as f64).ln().powf(power.unwrap_or(1.1))) as u32;
    default_scale(n)
}

pub fn sub_linear_scaling(n: u32) -> u32 {
    (n as f64 / ((n as f64).sqrt())) as u32
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

pub fn ln_power_scaling(n: u32, power: Option<f64>) -> u32 {
    let n = n as f64;
    n.ln().powf(power.unwrap_or(2.0)) as u32
}

pub fn exp_scaling(n: u32) -> u64 {
    let ee = n as f64;
    let exp = ((ee.ln() + 1.0).ln()).min(1.0);
    ee.powf(E * exp) as u64
}

#[instrument(ret, level = Level::TRACE)]
pub fn enemy_exp_scaling(n: u32) -> u64 {
    level_up_scaling(n, Some(1.1))
}

#[instrument(ret, level = Level::TRACE)]
pub fn enemy_defense_scaling(n: u32, grade: u32) -> u64 {
    let n = n as f64;
    let grade = grade as f64;
    (n.powf(1.3) + grade.powf(1.3)) as u64
}
#[instrument(ret, level = Level::TRACE)]
pub fn enemy_damage_scaling(n: u32) -> i32 {
    let n = n as f64;
    n.powf(1.1) as i32
}

#[instrument(ret, level = Level::TRACE)]
pub fn level_up_scaling(n: u32, power: Option<f64>) -> u64 {
    let n = n as f64;
    n.powf(power.unwrap_or(1.3)) as u64
}
#[instrument(ret, level = Level::TRACE)]
pub fn dodge_scaling(n: i32) -> f64 {
    let n = n + 5;
    100.0 - (100.0 * E.powf(-(n as f64 / 5.0).ln() / 4.0))
}

#[instrument(ret, level = Level::TRACE)]
pub fn armor_scaling(n: i32) -> f64 {
    let n = n + 5;
    100.0 - (100.0 * E.powf(-(n as f64 / 5.0).ln() / 3.0))
}

#[instrument(ret, level = Level::TRACE)]
pub fn resistance_scaling(n: i32) -> f64 {
    let n = n + 9;
    100.0 - (100.0 * E.powf(-(n as f64 / 9.0).ln() / 3.0))
}

trait ValidEnum {
    fn valid() -> String;
}

trait EnemyEvents {
    fn grade(&self) -> crate::enemy::MobGrade;
    fn actions(&self) -> Vec<crate::skill::MobAction>;

    fn alignment(&self) -> crate::unit::Alignment;

    fn vulnerability(&self) -> Option<damage::DamageType>;
}

#[cfg(test)]
mod test {
    use crate::{
        character::Character,
        damage::Defense,
        enemy::{Enemy, Mob},
    };

    #[test]
    fn enemy_legendary_defense_scaling() {
        let mut charater: Character = Default::default();
        charater.level = 30;
        let enemy: Mob = Mob::Eldragor;
        let enemy: Enemy = enemy.generate(charater.level);
        let defense: Defense = (&enemy).into();

        let mitigation = defense.defense(crate::damage::ResistCategories::Physical);
        assert!(
            mitigation > 72.,
            "Defense: {:?} mitigation {:3}",
            defense,
            mitigation
        );
        assert!(
            mitigation < 80.,
            "Defense: {:?} mitigation {:3}",
            defense,
            mitigation
        );
    }

    #[test]
    fn enemy_legendary_defense_scaling_high_level() {
        let mut charater: Character = Default::default();
        charater.level = 150;
        let enemy: Mob = Mob::Eldragor;
        let enemy: Enemy = enemy.generate(charater.level);
        let defense: Defense = (&enemy).into();

        let mitigation = defense.defense(crate::damage::ResistCategories::Physical);
        assert!(
            mitigation > 80.,
            "Defense: {:?} mitigation {}",
            defense,
            mitigation
        );
        assert!(
            mitigation < 99.,
            "Defense: {:?} mitigation {}",
            defense,
            mitigation
        );
    }

    #[test]
    fn enemy_common_defense_scaling() {
        let mut charater: Character = Default::default();
        charater.level = 30;
        let enemy: Mob = Mob::Orc;
        let enemy: Enemy = enemy.generate(charater.level);
        let defense: Defense = (&enemy).into();
        let mitigation = defense.defense(crate::damage::ResistCategories::Physical);
        assert!(
            mitigation > 33.,
            "Defense: {:?} mitigation {}",
            defense,
            mitigation
        );
        assert!(
            mitigation < 70.,
            "Defense: {:?} mitigation {}",
            defense,
            mitigation
        );
    }
}
