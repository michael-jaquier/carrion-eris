use crate::database::surreal::consumer::SurrealConsumer;
use crate::database::surreal::producer::SurrealProducer;
use crate::enemies::{Enemy, Mob};
use crate::player::{BattleInfo, Character, Classes};
use crate::CarrionResult;
use rand::{random, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::future::Future;
use tracing::{debug, info, warn};
use tracing_subscriber::fmt::{format, init};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Notification {
    TraitReady,
}

impl Display for Notification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Notification::TraitReady => write!(f, "You have a trait point to spend"),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToNotify(pub HashMap<u64, Notification>);

impl ToNotify {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, user_id: u64, notification: Notification) {
        self.0.insert(user_id, notification);
    }

    pub fn remove(&mut self, user_id: u64) {
        self.0.remove(&user_id);
    }

    pub fn get(&self, user_id: u64) -> Option<&Notification> {
        self.0.get(&user_id)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleResult {
    pub result: Vec<BattleInfo>,
}

impl Default for BattleResult {
    fn default() -> Self {
        Self { result: vec![] }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleResults {
    pub results: Vec<BattleResult>,
    pub notifications: ToNotify,
}

impl BattleResults {
    pub fn new(results: Vec<BattleResult>, notifications: ToNotify) -> Self {
        Self {
            results,
            notifications,
        }
    }

    pub fn append_result(&mut self, result: BattleResult) {
        self.results.push(result);
    }
}

impl Display for BattleResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let total_damage = self
            .result
            .iter()
            .map(|r| r.damage)
            .fold(0, |acc, d| acc + d);
        let critical = self
            .result
            .iter()
            .map(|r| r.critical)
            .filter(|c| *c == true)
            .count();
        let kill = self
            .result
            .iter()
            .map(|r| r.kill)
            .filter(|k| *k == true)
            .count();
        let leveled_up = self
            .result
            .iter()
            .map(|r| r.leveled_up)
            .filter(|l| *l == true)
            .count();
        let player_name = self.result[0].player_name.clone();
        let monster_name = self.result[0].monster_name.clone();
        let action = self.result[0].action.clone();
        let remaining_monster_hp = self.result.last().unwrap().monster_hp;
        let traits = self.result.last().unwrap().traits_available.clone();
        let mut string = String::new();
        string.push_str("\n🗡️");
        string.push_str("\n\t");
        string.push_str("🎲\t");
        string.push_str(&player_name);
        string.push_str(" attacked the ");
        string.push_str(&monster_name);
        string.push_str(" with ");
        string.push_str(&action.to_string());
        string.push_str(&format!(" {} times", self.result.len()));
        string.push_str(" dealing ");
        string.push_str(&total_damage.to_string());
        string.push_str(" damage!");
        string.push_str("\t🎲");

        if critical > 0 {
            string.push_str("\n\t");
            string.push_str("💥️\t");
            string.push_str(&format!("Scored {} Critical hits!", critical));
            string.push_str("\t💥️");
        }

        if kill > 0 {
            string.push_str("\n\t");
            string.push_str("☠️\t");
            string.push_str("Killing blow");
            string.push_str("\t☠️");
        } else {
            string.push_str("\n\t");
            string.push_str("🩸\t");
            string.push_str(&format!("{} was wounded resting a turn", player_name));
            string.push_str("\t🩸");
            string.push_str("\n\t");
            string.push_str("⬜\t");
            string.push_str(&format!(
                "{} has {} hp remaining",
                monster_name, remaining_monster_hp
            ));
            string.push_str("\t⬜");
        }

        if leveled_up > 0 {
            string.push_str("\n\t");
            string.push_str("🎉\t");
            string.push_str(&format!("Leveled up {} times!", leveled_up));
            string.push_str("\t🎉")
        }

        if traits > 0 {
            string.push_str("\n\t");
            string.push_str("🏺\t");
            string.push_str(&format!("You have trait {} points to spend!", traits));
            string.push_str("\t🏺")
        }
        string.push_str("\n🗡️\n");

        write!(f, "{}", string)
    }
}

async fn single_turn(character: &mut Character, enemy: &mut Enemy) -> BattleInfo {
    let result = character.player_action(enemy);
    // If the enemy is dead they should not act
    if enemy.alive() {
        character.enemy_action(&enemy);
    }
    result
}

async fn battle(mut character: &mut Character) -> BattleResult {
    let old_enemy = SurrealConsumer::get_enemy(&character).await.unwrap();
    let mut enemy = match old_enemy {
        None => {
            let mob_choice: Mob = random();
            mob_choice.generate(&character)
        }
        Some(e) => e,
    };
    let mut battle_info = vec![];
    while enemy.alive() && character.hp > 0 {
        let result = single_turn(&mut character, &mut enemy).await;
        battle_info.push(result);
    }

    if character.hp <= 0 {
        SurrealProducer::store_enemy(enemy.clone(), &character)
            .await
            .expect("Failed to store enemy");
    } else {
        SurrealProducer::delete_enemy(&character)
            .await
            .expect("Failed to delete enemy");
    }
    BattleResult {
        result: battle_info,
    }
}

pub async fn all_battle() -> BattleResults {
    debug!("All Battle!");
    let characters = SurrealConsumer::get_all_characters().await;
    debug!("Characters: {:?}", characters);
    let mut results = BattleResults::new(vec![], ToNotify::new());
    match characters {
        Ok(characters) => {
            for mut character in characters {
                if character.hp <= 0 {
                    character.rest();
                    debug!("Letting downed character: {:?} rest up", character);
                    let _ = SurrealProducer::create_or_update_character(character).await;
                    continue;
                }
                let result = battle(&mut character).await;
                results.append_result(result);

                if character.available_traits > 0 {
                    results
                        .notifications
                        .add(character.user_id, Notification::TraitReady);
                }

                SurrealProducer::create_or_update_character(character)
                    .await
                    .expect("Failed to update character");
            }
        }
        Err(e) => {
            warn!("Failed to get characters: {}", e)
        }
    }

    results
}

pub async fn all_notify() -> Vec<String> {
    let characters = SurrealConsumer::get_all_characters().await;
    let mut r = vec![];
    match characters {
        Ok(characters) => {
            for character in characters {
                if character.available_traits > 0 {
                    r.push(format!(
                        "{} has {} trait points to spend",
                        character.name, character.available_traits
                    ));
                }
            }
        }
        Err(_) => {
            info!("Failed to get characters");
        }
    }
    r
}
