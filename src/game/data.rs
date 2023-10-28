use crate::battle::BattleResult;
use crate::database::{Consumer, Database};
use crate::{database::Producer, item::Items};

use crate::character::Character;
use crate::enemy::Mob;
use crate::game::character_data::CharacterData;
use crate::game::mutations::Mutations;
use crate::game_loop::BUFFER;
use crate::BattleInfo;
use rand::random;
use std::collections::HashMap;
use tracing::{info, trace, warn};

pub struct GameData {
    pub characters: HashMap<u64, CharacterData>,
    producer: Box<dyn Producer + Sync + Send>,
    consumer: Box<dyn Consumer + Sync + Send>,
    database: Database,
}

impl Default for GameData {
    fn default() -> Self {
        Self::new()
    }
}

impl GameData {
    pub async fn init(&mut self) {
        let characters = self.consumer.get_all_characters().await.unwrap_or_default();

        let mut game_data = HashMap::new();
        for c in characters {
            let character_data = CharacterData::init(&c, Database::Surreal).await;
            game_data.insert(c.user_id, character_data);
        }
        self.characters = game_data;
        self.activate_enemies();
    }

    pub fn new() -> Self {
        let database = Database::Surreal;
        Self {
            characters: HashMap::new(),
            producer: database.get_producer(),
            consumer: database.get_consumer(),
            database,
        }
    }

    pub fn get_character(&self, user_id: u64) -> Option<Character> {
        self.characters.get(&user_id).map(|c| c.character.clone())
    }

    pub fn get_items(&self, user_id: u64) -> Option<Items> {
        self.characters.get(&user_id).map(|c| c.items.clone())
    }

    fn activate_enemies(&mut self) {
        for character in self.characters.values_mut() {
            if character.active_enemy.is_some() {
                return;
            }

            if character.enemies.is_empty() {
                let mob: Mob = random();
                let enemy = mob.generate(&character.character);
                character.active_enemy = Some(enemy.clone());
                return;
            }

            let enemy = character.enemies.remove(0);
            let enemy = enemy.generate(&character.character);
            character.active_enemy = Some(enemy.clone());
        }
    }

    pub async fn battle(&self) -> BattleResult {
        let mut battles = BattleResult::default();

        for c in self.characters.values() {
            let enemy = c.active_enemy.as_ref().unwrap();
            let mut battle_info = BattleInfo::begin(&c.character, enemy);
            while enemy.health > battle_info.damage_dealt
                && c.character.hp > battle_info.damage_taken
            {
                c.character.player_attack(enemy, &mut battle_info);
                if enemy.health > battle_info.damage_dealt {
                    c.character.enemy_attack(enemy, &mut battle_info);
                }
            }

            GameData::apply_battle_info(&battle_info, c).await;
            battles.append_result(battle_info);
        }
        battles
    }

    pub async fn apply_battle_info(battle_info: &BattleInfo, character: &CharacterData) {
        let mut write_buffer = BUFFER.write().await;
        write_buffer.add(Mutations::NewItems(character.user_id, battle_info.into()));
        write_buffer.add(Mutations::UpdateEnemies(
            character.user_id,
            battle_info.clone(),
        ));
        write_buffer.add(Mutations::UpdatePlayer(
            character.user_id,
            battle_info.clone(),
        ));
    }

    pub async fn apply_global_mutations(&mut self, buffer: Vec<Mutations>) {
        for mutation in &buffer {
            match mutation {
                Mutations::Delete(user_id) => {
                    self.characters.remove(user_id);
                    let _ = self.producer.delete_character(*user_id).await.map_err(|e| {
                        warn!("Failed to delete character: {:?}", e);
                    });
                    let _ = self
                        .producer
                        .delete_character_skills(*user_id)
                        .await
                        .map_err(|e| {
                            warn!("Failed to delete character skills: {:?}", e);
                        });
                    let _ = self.producer.delete_mob_queue(*user_id).await.map_err(|e| {
                        warn!("Failed to delete related: {:?}", e);
                    });

                    let _ = self
                        .producer
                        .delete_user_items(*user_id)
                        .await
                        .map_err(|e| {
                            warn!("Failed to delete user items: {:?}", e);
                        });

                    info!("Deleted character: {}", user_id);
                }

                Mutations::Create(character) => {
                    let character_data = CharacterData::init(character, self.database).await;
                    self.characters.insert(character.user_id, character_data);
                    let _ = self
                        .producer
                        .create_character(*character.clone())
                        .await
                        .map_err(|e| {
                            warn!("Failed to store character: {:?}", e);
                        });
                    self.activate_enemies();
                    info!("Created character: {}", character.user_id);
                }

                Mutations::SynchronizeEnemies(user_id) => {
                    let now = tokio::time::Instant::now();
                    let character = self.characters.get(user_id);
                    if character.is_none() {
                        continue;
                    }
                    let character = character.unwrap();
                    let _ = self
                        .producer
                        .store_mob_queue(&character.character, character.enemies.clone())
                        .await
                        .map_err(|e| {
                            warn!("Failed to store enemies: {:?}", e);
                        });
                    trace!("Stored enemies in {:?}", now.elapsed());
                }

                Mutations::SynchronizeItems(user_id) => {
                    let now = tokio::time::Instant::now();
                    let character = self.characters.get(user_id);
                    if character.is_none() {
                        continue;
                    }
                    let character = character.unwrap();
                    let _ = self
                        .producer
                        .store_user_items(character.items.clone(), *user_id)
                        .await
                        .map_err(|e| {
                            warn!("Failed to store items: {:?}", e);
                        });
                    trace!("Stored items in {:?}", now.elapsed());
                }

                _ => {}
            }
        }
    }

    pub async fn apply_mutations(&mut self) {
        {
            let buffer = BUFFER.read().await.mutations.clone();
            for mutation in buffer {
                if let Some(c) = self.characters.get_mut(mutation.user_id()) {
                    c.apply_mutation(mutation).await;
                }
            }
        }

        let buffer = BUFFER.read().await.mutations.clone();
        self.apply_global_mutations(buffer).await;
    }

    pub async fn synchronize_db(&self) {
        let now = tokio::time::Instant::now();
        for character_data in self.characters.values() {
            let character = character_data.character.clone();

            let _ = self
                .producer
                .create_or_update_character(character)
                .await
                .map_err(|e| {
                    warn!("Failed to update character: {:?}", e);
                });
            trace!("Stored character in {:?}", now.elapsed());
        }
    }
}
