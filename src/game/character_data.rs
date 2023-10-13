use crate::constructed::ItemsWeHave;
use crate::database::{Consumer, Database, Producer};
use crate::enemies::{Enemy, Mob};
use crate::game::mutations::Mutations;
use crate::game_loop::BUFFER;
use crate::items::Items;
use crate::player::{Character, SkillSet};
use rand::random;
use std::collections::HashSet;
use tracing::{info, warn};

pub struct CharacterData {
    pub character: Character,
    pub enemies: Vec<Mob>,
    pub items: Items,
    pub user_id: u64,
    pub active_enemy: Option<Enemy>,
    producer: Box<dyn Producer + Sync + Send>,
    consumer: Box<dyn Consumer + Sync + Send>,
}

impl CharacterData {
    pub async fn init(character: &Character, database: Database) -> CharacterData {
        let producer = database.get_producer();
        let consumer = database.get_consumer();
        let items = consumer
            .get_items(character.user_id)
            .await
            .unwrap_or_default();
        let enemies = consumer
            .get_related_mobs(character)
            .await
            .expect("Unable to get related enemies from DB");
        let active_enemy = consumer
            .get_enemy(character)
            .await
            .expect("Unable to get active enemy from DB");

        info!("Active Enemy: {:?}", active_enemy);
        info!("Enemies: {:?}", enemies);
        info!("Items: {:?}", items);
        CharacterData {
            character: character.clone(),
            enemies,
            items: items.unwrap_or(Items::default()),
            user_id: character.user_id,
            active_enemy,
            consumer,
            producer,
        }
    }

    pub async fn apply_mutation(&mut self, mutation: Mutations) {
        match mutation {
            Mutations::Skill(_, skill) => {
                let _ = self
                    .producer
                    .create_or_update_skill(
                        self.character.current_skill.clone(),
                        self.character.user_id,
                    )
                    .await
                    .map_err(|e| {
                        warn!("Failed to update skill: {:?}", e);
                    });
                if let Ok(Some(known_skill)) =
                    self.consumer.get_skill(&self.character, skill as u64).await
                {
                    self.character.current_skill = known_skill;
                } else {
                    self.character.current_skill = SkillSet::new(skill);
                }

                let _ = self
                    .producer
                    .set_current_skill(self.character.current_skill.clone(), self.character.user_id)
                    .await
                    .map_err(|e| {
                        warn!("Failed to set current skill: {:?}", e);
                    });
            }

            Mutations::Equip(user_id, item) => {
                let removed = self.items.remove(&item);
                if !removed {
                    return;
                }
                let old_item = self.character.equipment.equip(item);
                if let Some(old_item) = old_item {
                    self.items.push(old_item);
                }
                BUFFER
                    .write()
                    .await
                    .add(Mutations::SynchronizeItems(user_id))
            }

            Mutations::Trait(_, trait_) => {
                if self.character.available_traits == 0 {
                    return;
                }
                self.character.traits.insert(trait_);
                self.character.available_traits -= 1;
            }

            Mutations::AddEnemy(user_id, mob, count) => {
                for _ in 0..count {
                    let cost = mob.generate(&self.character).cost();
                    self.items.gold -= cost;
                    if self.items.gold < cost {
                        self.items.gold += cost;
                        break;
                    }
                    self.enemies.push(mob);
                }
                BUFFER
                    .write()
                    .await
                    .add(Mutations::SynchronizeEnemies(user_id));
            }

            Mutations::Sell(user_id, slot, known_items) => {
                self.items
                    .sell_with_knowledge(slot.as_ref(), known_items.as_ref());
                BUFFER
                    .write()
                    .await
                    .add(Mutations::SynchronizeItems(user_id))
            }

            Mutations::NewItems(user_id, items) => {
                let unset_items: HashSet<ItemsWeHave> = items
                    .iter()
                    .filter_map(|item| self.character.equipment.auto_equip(*item))
                    .collect();

                self.items += Items::new(unset_items, items.gold);
                BUFFER
                    .write()
                    .await
                    .add(Mutations::SynchronizeItems(user_id))
            }

            Mutations::UpdateEnemies(_user_id, battle_info) => {
                let enemy = self.active_enemy.as_mut().unwrap();
                enemy.health -= battle_info.damage_dealt;
                enemy.health += battle_info.enemy_healing;

                if enemy.health <= 0 {
                    if self.enemies.is_empty() {
                        let mob: Mob = random();
                        let enemy = mob.generate(&self.character);
                        self.active_enemy = Some(enemy.clone());
                        return;
                    }
                    let enemy = self.enemies.remove(0);
                    let enemy = enemy.generate(&self.character);
                    self.active_enemy = Some(enemy.clone());
                }
            }

            Mutations::UpdatePlayer(_user_id, battle_info) => {
                self.character.hp -= battle_info.damage_taken;
                self.character.hp += battle_info.player_healing;

                if self.character.hp <= 0 {
                    self.character.hp = self.character.max_hp as i32;
                    return;
                }

                self.character.experience += battle_info.experience_gained;
                self.character.try_level_up();
                self.character.try_trait_gain();
            }

            Mutations::UpdateSkills(_user_id, battle_info) => {
                self.character.current_skill.experience +=
                    battle_info.skill_experience_gained as u64;
                self.character.current_skill.try_level_up();
            }

            _ => {}
        }
    }
}
