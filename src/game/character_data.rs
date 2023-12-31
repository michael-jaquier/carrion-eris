use crate::character::Character;
use crate::database::Database;
use crate::enemy::{Enemy, Mob};
use crate::game::mutations::Mutations;

use crate::item::{IndividualItem, Items};
use crate::skill::SkillSet;
use rand::random;
use std::collections::HashSet;

use tracing::{info, trace, warn};

#[derive(Clone)]
pub struct CharacterData {
    pub character: Character,
    pub enemies: Vec<Mob>,
    pub items: Items,
    pub user_id: u64,
    pub active_enemy: Option<Enemy>,
    pub database: Database,
}

impl CharacterData {
    pub async fn init(character: &Character, database: Database) -> CharacterData {
        let _producer = database.get_producer();
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

        trace!("Active Enemy: {:?}", active_enemy);
        trace!("Enemies: {:?}", enemies);
        trace!("Items: {:?}", items);
        CharacterData {
            character: character.clone(),
            enemies,
            items: items.unwrap_or(Items::default()),
            user_id: character.user_id,
            active_enemy,
            database,
        }
    }

    pub async fn apply_mutation(&mut self, mutation: Mutations) {
        match mutation {
            Mutations::Skill(_, skill) => {
                let _ = self
                    .database
                    .get_producer()
                    .create_or_update_skill(
                        self.character.current_skill.clone(),
                        self.character.user_id,
                    )
                    .await
                    .map_err(|e| {
                        warn!("Failed to update skill: {:?}", e);
                    });
                if let Ok(Some(known_skill)) = self
                    .database
                    .get_consumer()
                    .get_skill(&self.character, skill as u64)
                    .await
                {
                    self.character.current_skill = known_skill;
                } else {
                    self.character.current_skill = SkillSet::new(skill);
                }

                let _ = self
                    .database
                    .get_producer()
                    .set_current_skill(self.character.current_skill.clone(), self.character.user_id)
                    .await
                    .map_err(|e| {
                        warn!("Failed to set current skill: {:?}", e);
                    });
            }

            Mutations::Equip(_user_id, item) => {
                let removed = self.items.remove(&item);
                if !removed {
                    return;
                }
                let old_item = self.character.equipment.equip(item);
                if let Some(old_item) = old_item {
                    self.items.push(old_item);
                }
            }

            Mutations::Trait(_, trait_) => {
                if self.character.available_traits == 0 {
                    return;
                }
                info!("Inserting trait: {:?}", trait_);
                let new_trait = self.character.insert_trait(trait_);

                if new_trait {
                    info!("Trait {} inserted", trait_);
                    self.character.available_traits -= 1;
                }
                info!("Available traits: {}", self.character.available_traits);
                info!("Traits: {:?}", self.character.get_traits());
            }

            Mutations::AddEnemy(_user_id, mob, count) => {
                for _ in 0..count {
                    let cost = mob.generate(self.character.level).cost();
                    self.items.gold = self.items.gold.saturating_sub(cost);
                    if self.items.gold < cost {
                        self.items.gold += cost;
                        break;
                    }
                    self.enemies.push(mob);
                }
            }

            Mutations::Sell(_user_id, slot, known_items) => {
                self.items
                    .sell_with_knowledge(slot.as_ref(), known_items.as_ref());
            }

            Mutations::NewItems(_user_id, items) => {
                let unset_items: HashSet<IndividualItem> = items
                    .iter()
                    .filter_map(|item| self.character.equipment.auto_equip(item.clone()))
                    .collect();

                let return_items = self
                    .character
                    .equipment
                    .boost(Items::new(unset_items, 0), self.character.clone());

                self.items += Items::new(return_items, items.gold);
            }

            Mutations::UpdateEnemies(_user_id, battle_info) => {
                let enemy = self.active_enemy.as_mut().unwrap();
                enemy.health -= battle_info.player_damage;
                if battle_info.enemy_healing > 0 {
                    let max_heal = enemy.health / 4;
                    let heal = battle_info.enemy_healing.min(max_heal);
                    enemy.health += heal;
                }
                enemy.health = enemy.health.min(enemy.max_health() as i32);

                let enemy_level = if battle_info.enemy_damage == 0 {
                    battle_info.enemy_level + 3
                } else {
                    (battle_info.enemy_level / 2).max(self.character.level)
                };
                trace!(
                    "Enemy Level: {enemy_level} Player Level: {}",
                    self.character.level
                );

                if battle_info.enemy_killed {
                    if self.enemies.is_empty() {
                        let mob: Mob = random();
                        let enemy = mob.generate(enemy_level);
                        self.active_enemy = Some(enemy.clone());
                        return;
                    }
                    let enemy = self.enemies.remove(0);

                    let enemy = enemy.generate(enemy_level);
                    self.active_enemy = Some(enemy.clone());
                }
            }

            Mutations::UpdatePlayer(_user_id, battle_info) => {
                self.character.hp -= battle_info.enemy_damage;
                self.character.hp += battle_info.player_healing;

                // Heal after battle
                if self.character.hp <= 0 {
                    self.character.hp = self.character.max_hp as i32;
                    return;
                } else {
                    self.character.hp += (self.character.max_hp / 4) as i32;
                    self.character.hp = self.character.hp.min(self.character.max_hp as i32);
                }

                self.character.experience += battle_info.experience_gained;
                self.character.try_level_up();
                self.character.try_trait_gain();
            }

            Mutations::UpdateSkills(_user_id, battle_info) => {
                self.character.current_skill.experience += battle_info.skill_experience_gained;
                self.character.current_skill.try_level_up();
            }

            _ => {}
        }
    }
}
