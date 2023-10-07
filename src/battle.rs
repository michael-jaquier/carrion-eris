use crate::database::surreal::consumer::SurrealConsumer;
use crate::database::surreal::producer::SurrealProducer;
use crate::enemies::{Enemy, Mob, MobGrade};
use crate::player::{Character, SkillSet};

use rand::random;
use serde::{Deserialize, Serialize};

use std::fmt::Display;

use crate::{BattleInfo, EnemyEvents};
use tracing::{debug, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BattleResult {
    pub result: Vec<BattleInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleResults {
    pub results: Vec<BattleInfo>,
}

impl BattleResults {
    pub fn new(results: Vec<BattleInfo>) -> Self {
        Self { results }
    }

    pub fn append_result(&mut self, result: BattleInfo) {
        self.results.push(result);
    }
}

impl Display for BattleResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for result in &self.result {
            string.push_str(&result.to_string());
            string.push('n')
        }
        write!(f, "{}", string)
    }
}

async fn single_turn(character: &mut Character, enemy: &mut Enemy, battle_info: &mut BattleInfo) {
    character.player_attack(enemy, battle_info);
    // If the enemy is dead they should not act
    if enemy.alive() {
        character.enemy_attack(enemy, battle_info);
    }
}

async fn battle(character: &mut Character) -> BattleInfo {
    let mut enemy;
    let mut enemy_id = None;
    if let Some((e, t)) = SurrealConsumer::get_related_enemies(character)
        .await
        .unwrap()
    {
        enemy = e.clone();
        enemy_id = Option::from(t.clone());
    } else {
        let mut mob_choice: Mob = random();
        while character.level < 10 && mob_choice.grade() == MobGrade::Boss {
            mob_choice = random();
        }
        enemy = mob_choice.generate(character);
    }
    info!(
        "Enemy: Health: {}, Level: {}, Experience: {}",
        enemy.health, enemy.level, enemy.experience
    );
    let mut battle_info = BattleInfo::begin(character, &enemy);
    while enemy.alive() && character.hp > 0 {
        single_turn(character, &mut enemy, &mut battle_info).await;
    }

    SurrealProducer::store_related_enemy(character, &enemy, enemy_id)
        .await
        .expect("Failed to store enemy");
    battle_info.next_level = character.experience_to_next_level() - character.experience;
    if !battle_info.item_gained.is_empty() {
        if let Some(mut items) = SurrealConsumer::get_items(character.user_id)
            .await
            .expect("Failed to get items")
        {
            for item in &battle_info.item_gained {
                if let Some(store_item) = character.equipment.auto_equip(*item) {
                    items.push(store_item);
                }
            }

            SurrealProducer::store_user_items(items, character.user_id)
                .await
                .expect("Failed to store items");
        }
    }
    battle_info
}

pub async fn all_battle() -> BattleResults {
    debug!("All Battle!");
    let characters = SurrealConsumer::get_all_characters().await;
    debug!("Characters: {:?}", characters);
    let mut results = BattleResults::new(vec![]);
    match characters {
        Ok(characters) => {
            for mut character in characters {
                if character.hp <= 0 {
                    character.rest();
                    debug!("Letting downed character: {:?} rest up", character);
                    let _ = SurrealProducer::create_or_update_character(character).await;
                    continue;
                }

                let character_skill = SurrealConsumer::get_skill(&character, 999)
                    .await
                    .expect("Failed to get skill");
                character.current_skill =
                    character_skill.unwrap_or(SkillSet::new(character.class.action()));

                let result = battle(&mut character).await;

                SurrealProducer::patch_user_gold(result.gold_gained, character.user_id, false)
                    .await
                    .expect("Failed to patch gold");

                if character.hp > character.max_hp as i32 {
                    warn!("Character: {:?} has more hp than max_hp", character)
                }

                SurrealProducer::set_current_skill(character.current_skill.clone(), &character)
                    .await
                    .expect("Failed to set current skill");

                SurrealProducer::create_or_update_skill(
                    character.current_skill.clone(),
                    &character,
                )
                .await
                .expect("Failed to update skill");

                SurrealProducer::create_or_update_character(character)
                    .await
                    .expect("Failed to update character");

                results.append_result(result);
            }
        }
        Err(e) => {
            warn!("Failed to get characters: {}", e)
        }
    }
    results
}
