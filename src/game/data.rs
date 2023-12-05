use crate::battle::BattleResult;
use crate::database::{Consumer, Database};
use crate::{database::Producer, item::Items};

use crate::character::Character;
use crate::enemy::Mob;
use crate::game::character_data::CharacterData;
use crate::game::mutations::Mutations;
use crate::BattleInfo;
use dashmap::DashMap;
use rand::random;

use tracing::{info, trace, warn};

use super::buffer::Buffer;

pub struct GameData {
    pub characters: DashMap<u64, CharacterData>,
    producer: Box<dyn Producer + Sync + Send>,
    consumer: Box<dyn Consumer + Sync + Send>,
    database: Database,
    buffer: Buffer,
}

impl Default for GameData {
    fn default() -> Self {
        Self::new(Database::Mock)
    }
}

impl GameData {
    pub async fn init(&mut self) {
        let characters = self.consumer.get_all_characters().await.unwrap_or_default();

        let game_data = DashMap::new();
        for c in characters {
            let character_data = CharacterData::init(&c, self.database).await;
            game_data.insert(c.user_id, character_data);
        }
        self.characters = game_data.clone();
        self.activate_enemies();
    }

    pub fn new(database: Database) -> Self {
        Self {
            characters: DashMap::new(),
            producer: database.get_producer(),
            consumer: database.get_consumer(),
            database,
            buffer: Buffer::new(),
        }
    }

    pub(crate) fn get_buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn get_character(&self, user_id: u64) -> Option<Character> {
        self.characters.get(&user_id).map(|c| c.character.clone())
    }

    pub fn get_items(&self, user_id: u64) -> Option<Items> {
        self.characters.get(&user_id).map(|c| c.items.clone())
    }

    fn activate_enemies(&self) {
        for mut character in self.characters.iter_mut() {
            if character.active_enemy.is_some() {
                return;
            }

            if character.enemies.is_empty() {
                let mob: Mob = random();
                let enemy = mob.generate(character.character.level);
                character.active_enemy = Some(enemy.clone());
                return;
            }

            let enemy = character.enemies.remove(0);
            let enemy = enemy.generate(character.character.level);
            character.active_enemy = Some(enemy.clone());
        }
    }

    pub async fn battle(&self, character: u64) -> BattleResult {
        let mut battles = BattleResult::default();
        if let Some(c) = self.characters.get(&character) {
            let enemy = c.active_enemy.as_ref().unwrap();
            let mut battle_info = BattleInfo::begin(&c.character, enemy);
            while !battle_info.enemy_killed && !battle_info.player_killed {
                c.character.player_attack(enemy, &mut battle_info);

                if !battle_info.enemy_killed {
                    c.character.enemy_attack(enemy, &mut battle_info);
                }
            }
            self.apply_battle_info(&battle_info, c.user_id);
            battles.append_result(battle_info);
        }

        battles
    }

    pub fn apply_battle_info(&self, battle_info: &BattleInfo, character_id: u64) {
        let mutations = vec![
            Mutations::UpdatePlayer(character_id, battle_info.clone()),
            Mutations::UpdateEnemies(character_id, battle_info.clone()),
            Mutations::NewItems(character_id, battle_info.into()),
        ];
        self.buffer.extend(mutations);
    }

    pub async fn apply_global_mutations(&self, buffer: Vec<Mutations>) {
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

                _ => {}
            }
        }
    }

    pub async fn apply_mutations(&self, character: u64) {
        trace!("Applying Mutations");

        if let Some(buffer) = self.buffer.get(&character) {
            for mutation in buffer.iter() {
                if let Some(mut c) = self.characters.get_mut(mutation.user_id()) {
                    c.apply_mutation(mutation.clone()).await;
                }
            }
        }

        if let Some(buffer) = self.buffer.get(&character) {
            self.apply_global_mutations(buffer.clone()).await;
        }
    }

    pub async fn synchronize_db(&self) {
        for character in self.characters.iter() {
            let _ = self
                .producer
                .store_mob_queue(&character.character, character.enemies.clone())
                .await
                .map_err(|e| {
                    warn!("Failed to store enemies: {:?}", e);
                });

            let _ = self
                .producer
                .store_user_items(character.items.clone(), character.user_id)
                .await
                .map_err(|e| {
                    warn!("Failed to store items: {:?}", e);
                });

            let character = character.character.clone();

            let _ = self
                .producer
                .create_or_update_character(character)
                .await
                .map_err(|e| {
                    warn!("Failed to update character: {:?}", e);
                });
        }
    }
}
