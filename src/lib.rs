pub mod battle;
pub mod classes;
pub mod commands;
pub mod database;
pub mod enemies;
pub mod mutators;
pub mod player;
pub mod traits;
pub mod units;

// Custom user data passed to all command functions
pub struct State {}
type CarrionResult<T> = Result<T, CarrionError>;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, State, Error>;

use crate::AdvantageState::Advantage;
use rand::Rng;
use serde::{Deserialize, Serialize};

use surrealdb::sql::Thing;
use thiserror::Error;

use crate::player::PlayerAction;

use std::fmt::{Display, Formatter};
use AdvantageState::Disadvantage;
use crate::Die::D20;

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
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct DieObject {
   die: Die,
   advantage: AdvantageState,
   success: u32,
   critical: u32,
}

impl DieObject {
    pub fn new(die: Die) -> Self {
        Self {
            die,
            advantage: AdvantageState::None,
            success: die.sides(),
            critical: die.sides(),
        }
    }
    pub fn roll(&self) -> u32 {
       self.die.roll(self.advantage)
    }
    pub fn crit(&self) -> bool {
       self.die.roll(self.advantage) >= self.critical
    }
    pub fn success(&self) -> bool {
       self.die.roll(self.advantage) >= self.success
    }

    pub fn set_success(&mut self, success: u32) {
        self.success = success;
    }
    pub fn set_critical(&mut self, critical: u32) {
        self.critical = critical;
    }

}

impl From<Die> for DieObject {
    fn from(die: Die) -> Self {
        Self::new(die)
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Copy, Debug)]
pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

impl Die {
    fn sides(&self) -> u32 {
        match self {
            Die::D4 => 4,
            Die::D6 => 6,
            Die::D8 => 8,
            Die::D10 => 10,
            Die::D12 => 12,
            Die::D20 => 20,
            Die::D100 => 100,
        }
    }
    fn roll(&self, advantage: AdvantageState) -> u32 {
        let mut rng = rand::thread_rng();
        let ranges = 1..self.sides() + 1;
        if advantage == Advantage {
            let roll1 = rng.gen_range(ranges.clone());
            let roll2 = rng.gen_range(ranges);
            return std::cmp::max(roll1, roll2);
        }
        if advantage == Disadvantage {
            let roll1 = rng.gen_range(ranges.clone());
            let roll2 = rng.gen_range(ranges);
            return std::cmp::min(roll1, roll2);
        }
        rng.gen_range(ranges)
    }

    fn success(
        &self,
        target: u32,
        advantage: AdvantageState,
    ) -> bool {
        let roll = self.roll(advantage);
        roll >= target
    }

    fn critical(&self, target: u32, advantage: AdvantageState) -> bool {
        let critical_dice = D20;
        let roll = critical_dice.roll(advantage);
        roll >= target
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Copy, Debug)]
pub enum AdvantageState {
    Advantage,
    Disadvantage,
    None,
}
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Dice {
    dice: Vec<DieObject>,
}

impl Dice {
    pub fn new(dice: Vec<DieObject>) -> Self {
        Self {
            dice,

        }
    }
    fn success_roll_all(&self) -> Vec<bool> {
        self.dice
            .iter()
            .map(|d| d.success())
            .collect()
    }
    pub fn success(&self) -> bool {
        self.success_roll_all().iter().any(|s| *s)
    }

    pub fn roll_sum(&self) -> u32 {
        self.dice.iter().map(|d| d.roll()).sum()
    }


    pub fn add_die(&mut self, die: Vec<DieObject>) {
        self.dice.extend(die);
    }

    pub fn advantage(&mut self) {
        self.dice.iter_mut().for_each(|d| d.advantage = Advantage);
    }
    pub fn disadvantage(&mut self) {
        self.dice.iter_mut().for_each(|d| d.advantage = Disadvantage);
    }


}

impl Default for Dice {
    fn default() -> Self {
        Self {
            dice: vec![Die::D20.into()],
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dice() {
        let dice = Dice::new(vec![Die::D20.into(); 3]);
        let result = dice.success();
        assert!(result);
    }
}
