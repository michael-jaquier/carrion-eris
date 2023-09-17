pub mod battle;
pub mod commands;
pub mod database;
pub mod enemies;
pub mod player;

// Custom user data passed to all command functions
pub struct State {}
type CarrionResult<T> = Result<T, CarrionError>;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, State, Error>;

use crate::AdvantageState::Advantage;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serenity::model::prelude::command::CommandOptionType::Attachment;
use std::fmt::Display;
use std::ops::{Add, Deref, Sub};
use surrealdb::sql::Thing;
use thiserror::Error;
use tracing::info;
use tracing::span::Attributes;
use AdvantageState::Disadvantage;

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

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialOrd, PartialEq)]
pub enum Attribute {
    Strength(u32),
    Intelligence(u32),
    Dexterity(u32),
    Constitution(u32),
    Wisdom(u32),
    Charisma(u32),
}

impl Attribute {
    pub fn absolute_difference(&self, other: &Self) -> i32 {
        **self as i32 - **other as i32
    }
    pub fn plus(&mut self, other: u32) {
        match self {
            Attribute::Strength(v) => *v += other,
            Attribute::Intelligence(v) => *v += other,
            Attribute::Dexterity(v) => *v += other,
            Attribute::Constitution(v) => *v += other,
            Attribute::Wisdom(v) => *v += other,
            Attribute::Charisma(v) => *v += other,
        }
    }

    pub fn minus(&mut self, other: u32) {
        match self {
            Attribute::Strength(v) => (*v).checked_sub(other).unwrap_or(0),
            Attribute::Intelligence(v) => (*v).checked_sub(other).unwrap_or(0),
            Attribute::Dexterity(v) => (*v).checked_sub(other).unwrap_or(0),
            Attribute::Constitution(v) => (*v).checked_sub(other).unwrap_or(0),
            Attribute::Wisdom(v) => (*v).checked_sub(other).unwrap_or(0),
            Attribute::Charisma(v) => (*v).checked_sub(other).unwrap_or(0),
        };
    }

    pub fn inner(&self) -> u32 {
        match self {
            Attribute::Strength(v) => *v,
            Attribute::Intelligence(v) => *v,
            Attribute::Dexterity(v) => *v,
            Attribute::Constitution(v) => *v,
            Attribute::Wisdom(v) => *v,
            Attribute::Charisma(v) => *v,
        }
    }
}

impl Deref for Attribute {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        match self {
            Attribute::Strength(v) => v,
            Attribute::Intelligence(v) => v,
            Attribute::Dexterity(v) => v,
            Attribute::Constitution(v) => v,
            Attribute::Wisdom(v) => v,
            Attribute::Charisma(v) => v,
        }
    }
}

impl Add for Attribute {
    type Output = Attribute;

    fn add(self, rhs: Self) -> Self::Output {
        let v = *self + *rhs;
        match self {
            Attribute::Strength(_) => Attribute::Strength(v),
            Attribute::Intelligence(_) => Attribute::Intelligence(v),
            Attribute::Dexterity(_) => Attribute::Dexterity(v),
            Attribute::Constitution(_) => Attribute::Constitution(v),
            Attribute::Wisdom(_) => Attribute::Wisdom(v),
            Attribute::Charisma(_) => Attribute::Charisma(v),
        }
    }
}

impl Sub for Attribute {
    type Output = Attribute;

    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.checked_sub(*rhs).unwrap_or(0);
        match self {
            Attribute::Strength(_) => Attribute::Strength(v),
            Attribute::Intelligence(_) => Attribute::Intelligence(v),
            Attribute::Dexterity(_) => Attribute::Dexterity(v),
            Attribute::Constitution(_) => Attribute::Constitution(v),
            Attribute::Wisdom(_) => Attribute::Wisdom(v),
            Attribute::Charisma(_) => Attribute::Charisma(v),
        }
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
        let ranges = (1..self.sides() + 1);
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
        critical_success: Option<u32>,
        advantage: AdvantageState,
    ) -> (bool, CriticalRole) {
        let roll = self.roll(advantage);
        let critical = match roll {
            r if r >= critical_success.unwrap_or(20) => CriticalRole::Success,
            r if r == 1 => CriticalRole::Failure,
            _ => CriticalRole::None,
        };
        (roll >= target, critical)
    }
}

type Role = (bool, CriticalRole);
type RoleResults = Vec<Role>;

#[derive(Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum CriticalRole {
    Success,
    Failure,
    None,
}
#[derive(Clone, PartialEq, Serialize, Deserialize, Copy, Debug)]
enum AdvantageState {
    Advantage,
    Disadvantage,
    None,
}
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Dice {
    pub dice: Die,
    pub number: u32,
    pub critical_success: u32,
    critical_failure: u32,
    advantage: AdvantageState,
    stored_target: Option<u32>,
}

impl Dice {
    pub fn new(dice: Die, number: u32, critical_success: Option<u32>) -> Self {
        Self {
            dice,
            number,
            critical_success: critical_success.unwrap_or(20),
            critical_failure: 1,
            advantage: AdvantageState::None,
            stored_target: None,
        }
    }
    fn success_roll(&self, target: u32) -> Role {
        self.dice
            .success(target, Some(self.critical_success), self.advantage)
    }
    fn success_roll_all(&self, target: u32) -> RoleResults {
        (0..self.number)
            .map(|_| self.success_roll(target))
            .collect()
    }
    pub fn success(&self, target: Option<u32>) -> bool {
        let target = target.unwrap_or(self.stored_target.unwrap_or(20));
        self.success_roll_all(target).iter().any(|(s, _)| *s)
    }

    pub fn roll_all(&self) -> u32 {
        (0..self.number)
            .map(|_| self.dice.roll(self.advantage))
            .sum()
    }

    pub fn advantage(&mut self) {
        self.advantage = Advantage;
    }

    pub fn disadvantage(&mut self) {
        self.advantage = Disadvantage;
    }

    pub fn set_target(&mut self, target: u32) {
        self.stored_target = Some(target);
    }
}

impl Default for Dice {
    fn default() -> Self {
        Self {
            dice: Die::D20,
            number: 1,
            critical_success: 20,
            critical_failure: 1,
            advantage: AdvantageState::None,
            stored_target: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dice() {
        let dice = Dice::new(Die::D20, 1, Some(20));
        let result = dice.success(Some(1));
        assert_eq!(result, true);
    }
}
